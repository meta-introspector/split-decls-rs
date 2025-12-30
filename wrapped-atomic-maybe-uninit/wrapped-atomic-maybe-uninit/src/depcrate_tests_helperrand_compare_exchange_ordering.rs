// Generated macro for rand_compare_exchange_ordering (function)
macro_rules! Depcrate_tests_helperrand_compare_exchange_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"rand_compare_exchange_ordering"}
// Dependencies: {}
pub (crate) fn rand_compare_exchange_ordering (rng : & mut fastrand :: Rng) -> (Ordering , Ordering) { COMPARE_EXCHANGE_ORDERINGS [rng . usize (0 .. COMPARE_EXCHANGE_ORDERINGS . len ())] }
};
}
