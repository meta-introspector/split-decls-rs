macro_rules! rand_load_ordering {
    () => {
        pub (crate) fn rand_load_ordering (rng : & mut fastrand :: Rng) -> Ordering { LOAD_ORDERINGS [rng . usize (0 .. LOAD_ORDERINGS . len ())] }
    };
}

rand_load_ordering!();