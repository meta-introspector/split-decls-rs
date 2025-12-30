// Generated macro for rand_load_ordering (function)
macro_rules! Depcrate_tests_helperrand_load_ordering {
() => {
// Module: crate::tests::helper
// Provides: {"rand_load_ordering"}
// Dependencies: {}
pub (crate) fn rand_load_ordering (rng : & mut fastrand :: Rng) -> Ordering { helper :: LOAD_ORDERINGS [rng . usize (0 .. helper :: LOAD_ORDERINGS . len ())] }
};
}
