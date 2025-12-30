// Generated macro for use_697 (use)
macro_rules! Depcrate_impuse_697 {
() => {
// Module: crate::imp
// Provides: {"use_697"}
// Dependencies: {}
# [cfg (all (target_arch = "riscv64" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , any (target_os = "linux" , target_os = "android") ,) ,) ,))] pub (crate) use self :: atomic128 :: riscv64 :: { AtomicI128 , AtomicU128 } ;
};
}
