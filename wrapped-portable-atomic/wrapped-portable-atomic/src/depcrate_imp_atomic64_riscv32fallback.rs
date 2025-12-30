// Generated macro for fallback (module)
macro_rules! Depcrate_imp_atomic64_riscv32fallback {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"fallback"}
// Dependencies: {}
# [cfg (not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")))] # [path = "../fallback/outline_atomics.rs"] mod fallback ;
};
}
