// Generated macro for use_695 (use)
macro_rules! Depcrate_impuse_695 {
() => {
// Module: crate::imp
// Provides: {"use_695"}
// Dependencies: {}
# [cfg (any (all (target_arch = "aarch64" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) ,) , all (target_arch = "arm64ec" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , not (portable_atomic_no_asm) ,) ,))] pub (crate) use self :: atomic128 :: aarch64 :: { AtomicI128 , AtomicU128 } ;
};
}
