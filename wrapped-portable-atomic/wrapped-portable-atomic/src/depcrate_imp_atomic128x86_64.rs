// Generated macro for x86_64 (module)
macro_rules! Depcrate_imp_atomic128x86_64 {
() => {
// Module: crate::imp::atomic128
// Provides: {"x86_64"}
// Dependencies: {}
# [cfg (all (target_arch = "x86_64" , not (all (any (miri , portable_atomic_sanitize_thread) , portable_atomic_no_cmpxchg16b_intrinsic)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , not (any (target_env = "sgx" , miri)) ,) ,) ,))] # [cfg_attr (any (miri , portable_atomic_sanitize_thread) , path = "intrinsics.rs")] pub (super) mod x86_64 ;
};
}
