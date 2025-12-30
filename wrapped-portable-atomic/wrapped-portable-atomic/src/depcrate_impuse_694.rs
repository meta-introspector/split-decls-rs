// Generated macro for use_694 (use)
macro_rules! Depcrate_impuse_694 {
() => {
// Module: crate::imp
// Provides: {"use_694"}
// Dependencies: {}
# [cfg (all (target_arch = "riscv32" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , any (target_os = "linux" , target_os = "android") ,) ,) ,))] pub (crate) use self :: atomic64 :: riscv32 :: { AtomicI64 , AtomicU64 } ;
};
}
