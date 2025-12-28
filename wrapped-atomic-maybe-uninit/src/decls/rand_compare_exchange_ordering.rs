macro_rules! rand_compare_exchange_ordering {
    () => {
        pub (crate) fn rand_compare_exchange_ordering (rng : & mut fastrand :: Rng) -> (Ordering , Ordering) { COMPARE_EXCHANGE_ORDERINGS [rng . usize (0 .. COMPARE_EXCHANGE_ORDERINGS . len ())] }
    };
}

rand_compare_exchange_ordering!()