use std::cmp::Ordering;
use std::collections::HashMap;

type Res = [u64; 2];

fn connect(g: &mut [u64], u: u64, v: u64) {
    g[u as usize] |= 1 << v;
}

pub fn demanding_money(money: Vec<u64>, roads: Vec<[u64; 2]>) -> Res {
    let n = money.len();
    let mut g: Vec<u64> = Vec::new();
    for _ in 0..n {
        g.push(0)
    }
    // println!("{:?}", g);
    for [u, v] in roads {
        connect(&mut g, u - 1, v - 1);
        connect(&mut g, v - 1, u - 1);
    }
    // println!("{:?}", g);

    let mut cache: HashMap<u64, Res> = HashMap::new();
    fn f(unvisited_bits: u64, g: &Vec<u64>, cache: &mut HashMap<u64, Res>, money: &Vec<u64>) -> Res {
        if unvisited_bits == 0 {
            [0_u64, 1_u64]
        } else if let Some(result) = cache.get(&unvisited_bits) {
            *result
        } else {
            let mut u = 0_u64;
            while u < (money.len() as u64) && unvisited_bits & 1 << u == 0 {
                u += 1;
            }
            let bits = unvisited_bits & !(1 << u);
            let adj = g[u as usize];
            let bits_no_neighbors = bits & !adj;
            let [current_sum, current_count] = f(bits_no_neighbors, g, cache, money);
            let sum = money[u as usize] + current_sum;
            let [alt_sum, alt_count] = f(bits, g, cache, money);
            let result = match sum.cmp(&alt_sum) {
                Ordering::Greater => [sum, current_count],
                Ordering::Equal => [sum, current_count + alt_count],
                Ordering::Less => [alt_sum, alt_count]
            };
            cache.insert(unvisited_bits, result);
            result
        }
    }

    let unvisited_bits = (1_u64 << (n as u64)) - 1;
    f(unvisited_bits, &g, &mut cache, &money)
}

#[cfg(test)]
mod tests {
    use super::demanding_money;

    fn test(money: Vec<u64>, roads: Vec<[u64; 2]>, expected: [u64; 2]) {
        let result = demanding_money(money, roads);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_0() {
        test(vec![6, 8, 2], vec![[1, 2], [3, 2]], [8, 2]);
    }

    #[test]
    fn test_1() {
        test(vec![0, 100, 50], vec![[2, 3]], [100, 2]);
    }

    #[test]
    fn test_2() {
        test(vec![40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40], vec![[2, 13], [13, 14], [4, 10], [2, 5], [2, 3]], [480, 4]);
    }

    #[test]
    fn test_3() {
        test(vec![0, 0, 50, 50, 100, 0, 0, 0, 100, 0, 0, 0, 100, 0, 0, 100, 50, 0, 0, 0, 100], vec![[4, 9], [5, 20], [3, 21], [7, 10], [3, 12], [3, 7]], [550, 3072]);
    }

    #[test]
    fn test_4() {
        test(vec![0, 100], vec![], [100, 2]);
    }

    #[test]
    fn test_5() {
        test(vec![0; 33], vec![[2, 5]], [0, 6442450944]);
    }

    #[test]
    fn test_6() {
        test(vec![100, 0, 100, 100, 100, 0, 100], vec![[3, 5], [2, 4], [5, 6], [4, 6], [1, 4]], [300, 8]);
    }

    #[test]
    fn test_7() {
        test(vec![75, 75, 50, 75, 100], vec![[3, 5], [2, 5], [2, 3], [4, 5], [1, 3], [2, 4]], [175, 1]);
    }

    #[test]
    fn test_8() {
        test(vec![0], vec![], [0, 2]);
    }

    #[test]
    fn test_9() {
        test(vec![0, 0, 0, 0], vec![], [0, 16]);
    }

    #[test]
    fn test_10() {
        test(vec![0, 0, 0], vec![], [0, 8]);
    }

    #[test]
    fn test_11() {
        test(vec![1, 1], vec![], [2, 1]);
    }

    #[test]
    fn test_12() {
        test(vec![1], vec![], [1, 1]);
    }

    #[test]
    fn test_13() {
        test(vec![1, 1], vec![[1, 2]], [1, 2]);
    }

    #[test]
    fn test_14() {
        test(vec![1, 2], vec![[1, 2]], [2, 1]);
    }

    #[test]
    fn test_15() {
        test(vec![1, 2, 3], vec![], [6, 1]);
    }

    #[test]
    fn test_16() {
        test(vec![], vec![], [0, 1]);
    }

    #[test]
    fn test_17() {
        test(
            vec![25, 25, 25, 75, 100, 25, 25, 0, 0, 100, 75, 0, 75, 50, 0, 50, 50, 25, 50, 25, 0, 50, 25, 75, 50, 100, 100, 100, 25, 100, 25, 0, 50, 50],
            vec![[31, 34], [7, 28], [19, 20], [8, 24], [13, 19], [10, 18], [18, 33], [7, 31], [9, 18], [17, 31], [1, 4], [10, 23], [1, 34], [5, 18], [13, 27], [5, 22], [17, 27], [10, 11], [29, 33], [21, 32], [6, 7], [12, 18], [1, 18], [18, 19], [3, 25], [2, 25], [10, 21], [4, 30], [2, 29], [20, 34], [7, 22], [5, 23], [28, 29], [6, 19], [7, 34], [14, 28], [1, 19], [7, 19], [11, 30], [23, 31], [29, 34], [18, 27], [18, 25], [4, 21], [12, 22], [20, 26], [19, 27], [4, 13], [9, 26], [23, 33]],
            [925, 16],
        );
    }
}
