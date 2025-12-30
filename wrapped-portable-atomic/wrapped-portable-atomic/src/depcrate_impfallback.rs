// Generated macro for fallback (module)
macro_rules! Depcrate_impfallback {
() => {
// Module: crate::imp
// Provides: {"fallback"}
// Dependencies: {}
# [cfg (feature = "fallback")] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (portable_atomic_no_atomic_cas)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (target_has_atomic = "ptr"))] # [cfg (any (test , not (any (all (target_arch = "aarch64" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) ,) , all (target_arch = "arm64ec" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , not (portable_atomic_no_asm) ,) , all (target_arch = "x86_64" , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") ,) , all (target_arch = "riscv64" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas") ,) , all (target_arch = "powerpc64" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , portable_atomic_unstable_asm_experimental_arch , any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,) ,) , all (target_arch = "s390x" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , not (portable_atomic_no_asm) ,) ,))))] mod fallback ;
};
}
