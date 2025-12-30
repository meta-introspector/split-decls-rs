// Generated macro for use_696 (use)
macro_rules! Depcrate_impuse_696 {
() => {
// Module: crate::imp
// Provides: {"use_696"}
// Dependencies: {}
# [cfg (all (target_arch = "x86_64" , not (all (any (miri , portable_atomic_sanitize_thread) , portable_atomic_no_cmpxchg16b_intrinsic)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , not (any (target_env = "sgx" , miri)) ,) ,) ,))] pub (crate) use self :: atomic128 :: x86_64 :: { AtomicI128 , AtomicU128 } ;
};
}
