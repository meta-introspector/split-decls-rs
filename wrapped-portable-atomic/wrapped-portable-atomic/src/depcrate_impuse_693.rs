// Generated macro for use_693 (use)
macro_rules! Depcrate_impuse_693 {
() => {
// Module: crate::imp
// Provides: {"use_693"}
// Dependencies: {}
# [cfg (feature = "fallback")] # [cfg (all (target_arch = "arm" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_os = "linux" , target_os = "android") , not (any (target_feature = "v6" , portable_atomic_target_feature = "v6")) , not (portable_atomic_no_outline_atomics) ,))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (portable_atomic_no_atomic_64))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (target_has_atomic = "64")))] pub (crate) use self :: atomic64 :: arm_linux :: { AtomicI64 , AtomicU64 } ;
};
}
