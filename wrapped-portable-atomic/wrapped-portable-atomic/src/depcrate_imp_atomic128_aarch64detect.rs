// Generated macro for detect (module)
macro_rules! Depcrate_imp_atomic128_aarch64detect {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"detect"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (test , not (any (target_feature = "lse" , portable_atomic_target_feature = "lse"))))] # [cfg (windows)] # [path = "../detect/aarch64_windows.rs"] mod detect ;
};
}
