// Generated macro for aarch64 (module)
macro_rules! Depcrate_imp_atomic128aarch64 {
() => {
// Module: crate::imp::atomic128
// Provides: {"aarch64"}
// Dependencies: {}
# [cfg (any (all (target_arch = "aarch64" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) ,) , all (target_arch = "arm64ec" , not (all (any (miri , portable_atomic_sanitize_thread) , not (portable_atomic_atomic_intrinsics) ,)) , not (portable_atomic_no_asm) ,) ,))] # [cfg_attr (any (miri , portable_atomic_sanitize_thread) , path = "intrinsics.rs")] pub (super) mod aarch64 ;
};
}
