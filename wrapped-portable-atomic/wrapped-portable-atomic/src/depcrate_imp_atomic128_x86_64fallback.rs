// Generated macro for fallback (module)
macro_rules! Depcrate_imp_atomic128_x86_64fallback {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"fallback"}
// Dependencies: {}
# [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b")))] # [path = "../fallback/outline_atomics.rs"] mod fallback ;
};
}
