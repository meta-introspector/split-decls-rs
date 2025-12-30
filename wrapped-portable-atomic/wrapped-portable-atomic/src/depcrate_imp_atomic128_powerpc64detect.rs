// Generated macro for detect (module)
macro_rules! Depcrate_imp_atomic128_powerpc64detect {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"detect"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (any (test , not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)) ,))] # [cfg (target_os = "aix")] # [cfg (not (portable_atomic_pre_llvm_20))] # [cfg (any (test , portable_atomic_outline_atomics))] # [path = "../detect/powerpc64_aix.rs"] mod detect ;
};
}
