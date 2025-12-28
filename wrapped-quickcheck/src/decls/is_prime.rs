macro_rules! is_prime {
    () => {
        fn is_prime (n : usize) -> bool { n != 0 && n != 1 && (2 ..) . take_while (| i | i * i <= n) . all (| i | n % i != 0) }
    };
}

is_prime!();