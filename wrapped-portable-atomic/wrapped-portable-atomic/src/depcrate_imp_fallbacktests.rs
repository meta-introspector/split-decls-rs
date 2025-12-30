// Generated macro for tests (module)
macro_rules! Depcrate_imp_fallbacktests {
() => {
// Module: crate::imp::fallback
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; cfg_no_fast_atomic_64 ! { test_atomic_int ! (i64) ; test_atomic_int ! (u64) ; } test_atomic_int ! (i128) ; test_atomic_int ! (u128) ; cfg_no_fast_atomic_64 ! { stress_test ! (u64) ; } stress_test ! (u128) ; }
};
}
