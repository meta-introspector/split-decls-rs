// Generated macro for fallback (module)
macro_rules! Depcrate_imp_atomic128_powerpc64fallback {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"fallback"}
// Dependencies: {}
# [cfg (not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)))] # [path = "../fallback/outline_atomics.rs"] mod fallback ;
};
}
