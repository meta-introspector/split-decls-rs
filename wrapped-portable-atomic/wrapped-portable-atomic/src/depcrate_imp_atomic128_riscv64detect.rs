// Generated macro for detect (module)
macro_rules! Depcrate_imp_atomic128_riscv64detect {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"detect"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (test , not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas"))))] # [cfg (any (target_os = "linux" , target_os = "android"))] # [path = "../detect/riscv_linux.rs"] mod detect ;
};
}
