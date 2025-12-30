// Generated macro for detect (module)
macro_rules! Depcrate_imp_atomic64_riscv32detect {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"detect"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (test , not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas"))))] # [cfg (any (target_os = "linux" , target_os = "android"))] # [path = "../detect/riscv_linux.rs"] mod detect ;
};
}
