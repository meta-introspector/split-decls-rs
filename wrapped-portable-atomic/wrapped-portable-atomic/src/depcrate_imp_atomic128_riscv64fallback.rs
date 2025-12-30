// Generated macro for fallback (module)
macro_rules! Depcrate_imp_atomic128_riscv64fallback {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"fallback"}
// Dependencies: {}
# [cfg (not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")))] # [path = "../fallback/outline_atomics.rs"] mod fallback ;
};
}
