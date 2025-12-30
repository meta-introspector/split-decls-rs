// Generated macro for powerpc64 (module)
macro_rules! Depcrate_imp_atomic128powerpc64 {
() => {
// Module: crate::imp::atomic128
// Provides: {"powerpc64"}
// Dependencies: {}
# [cfg (all (target_arch = "powerpc64" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , portable_atomic_unstable_asm_experimental_arch , any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , any (all (target_os = "linux" , any (all (target_env = "gnu" , any (target_endian = "little" , not (target_feature = "crt-static")) ,) , all (target_env = "musl" , any (not (target_feature = "crt-static") , feature = "std") ,) , target_env = "ohos" , all (target_env = "uclibc" , not (target_feature = "crt-static")) , portable_atomic_outline_atomics ,) ,) , target_os = "android" , all (target_os = "freebsd" , any (target_endian = "little" , not (target_feature = "crt-static") , portable_atomic_outline_atomics ,) ,) , target_os = "openbsd" , all (target_os = "aix" , not (portable_atomic_pre_llvm_20) , any (test , portable_atomic_outline_atomics) ,) ,) , not (any (miri , portable_atomic_sanitize_thread)) ,) ,) ,))] # [cfg_attr (any (miri , portable_atomic_sanitize_thread) , path = "intrinsics.rs")] pub (super) mod powerpc64 ;
};
}
