macro_rules! rand_swap_ordering {
    () => {
        pub (crate) fn rand_swap_ordering (rng : & mut fastrand :: Rng) -> Ordering { SWAP_ORDERINGS [rng . usize (0 .. SWAP_ORDERINGS . len ())] }
    };
}

rand_swap_ordering!()