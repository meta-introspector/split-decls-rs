// Generated macro for arm_linux (module)
macro_rules! Depcrate_imp_atomic64arm_linux {
() => {
// Module: crate::imp::atomic64
// Provides: {"arm_linux"}
// Dependencies: {}
# [cfg (feature = "fallback")] # [cfg (all (target_arch = "arm" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_os = "linux" , target_os = "android") , any (test , not (any (target_feature = "v6" , portable_atomic_target_feature = "v6"))) , not (portable_atomic_no_outline_atomics) ,))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (any (test , portable_atomic_no_atomic_64)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (test , not (target_has_atomic = "64"))))] pub (super) mod arm_linux ;
};
}
