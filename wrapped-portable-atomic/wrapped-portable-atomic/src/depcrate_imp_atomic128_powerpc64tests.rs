// Generated macro for tests (module)
macro_rules! Depcrate_imp_atomic128_powerpc64tests {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"tests"}
// Dependencies: {}
# [cfg (not (all (valgrind , target_arch = "powerpc64")))] # [cfg (test)] mod tests { use super :: * ; test_atomic_int ! (i128) ; test_atomic_int ! (u128) ; stress_test ! (u128) ; }
};
}
