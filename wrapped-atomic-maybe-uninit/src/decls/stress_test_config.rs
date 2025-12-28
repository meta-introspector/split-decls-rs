macro_rules! stress_test_config {
    () => {
        pub (crate) fn stress_test_config (rng : & mut fastrand :: Rng) -> (usize , usize) { let iterations = if cfg ! (debug_assertions) { 5_000 } else { 25_000 } ; let threads = if cfg ! (debug_assertions) { 2 } else { rng . usize (2 ..= 8) } ; std :: eprintln ! ("threads={threads}") ; (iterations , threads) }
    };
}

stress_test_config!()