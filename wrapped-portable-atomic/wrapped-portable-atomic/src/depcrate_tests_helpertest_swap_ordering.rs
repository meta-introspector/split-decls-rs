// Generated macro for test_swap_ordering (function)
macro_rules! Depcrate_tests_helpertest_swap_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"test_swap_ordering"}
// Dependencies: {}
pub (crate) fn test_swap_ordering < T : std :: fmt :: Debug > (f : impl Fn (Ordering) -> T) { for & order in & helper :: SWAP_ORDERINGS { f (order) ; } }
};
}
