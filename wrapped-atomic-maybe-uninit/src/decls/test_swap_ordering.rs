macro_rules! test_swap_ordering {
    () => {
        pub (crate) fn test_swap_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering) -> T) { for order in SWAP_ORDERINGS { f (order) ; } }
    };
}

test_swap_ordering!();