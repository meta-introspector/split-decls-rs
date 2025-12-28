macro_rules! exact_vec {
    () => {
        fn exact_vec < T > (capacity : usize) -> Vec < T > { let mut v = Vec :: new () ; v . reserve_exact (capacity) ; v }
    };
}

exact_vec!();