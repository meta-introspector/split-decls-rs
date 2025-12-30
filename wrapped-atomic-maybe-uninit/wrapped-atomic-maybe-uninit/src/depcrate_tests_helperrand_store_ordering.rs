// Generated macro for rand_store_ordering (function)
macro_rules! Depcrate_tests_helperrand_store_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"rand_store_ordering"}
// Dependencies: {}
pub (crate) fn rand_store_ordering (rng : & mut fastrand :: Rng) -> Ordering { STORE_ORDERINGS [rng . usize (0 .. STORE_ORDERINGS . len ())] }
};
}
