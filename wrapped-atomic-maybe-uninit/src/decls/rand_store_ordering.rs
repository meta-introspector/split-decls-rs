macro_rules! rand_store_ordering {
    () => {
        pub (crate) fn rand_store_ordering (rng : & mut fastrand :: Rng) -> Ordering { STORE_ORDERINGS [rng . usize (0 .. STORE_ORDERINGS . len ())] }
    };
}

rand_store_ordering!()