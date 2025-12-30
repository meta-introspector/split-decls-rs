// Generated macro for stress_test_config (function)
macro_rules! Depcrate_tests_helperstress_test_config {
() => {
// Module: crate::tests::helper
// Provides: {"stress_test_config"}
// Dependencies: {}
pub (crate) fn stress_test_config (rng : & mut fastrand :: Rng) -> (usize , usize) { let iterations = if cfg ! (miri) { 50 } else if cfg ! (debug_assertions) { 5_000 } else { 25_000 } ; let threads = if cfg ! (debug_assertions) { 2 } else { rng . usize (2 ..= 8) } ; std :: eprintln ! ("threads={}" , threads) ; (iterations , threads) }
};
}
