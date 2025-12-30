// Generated macro for detect (module)
macro_rules! Depcrate_imp_atomic128_x86_64detect {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"detect"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_outline_atomics))] # [cfg (not (target_env = "sgx"))] # [cfg_attr (not (target_feature = "sse") , cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b"))))] # [path = "../detect/x86_64.rs"] mod detect ;
};
}
