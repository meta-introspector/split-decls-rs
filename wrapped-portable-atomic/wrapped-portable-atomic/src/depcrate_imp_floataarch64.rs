// Generated macro for aarch64 (module)
macro_rules! Depcrate_imp_floataarch64 {
() => {
// Module: crate::imp::float
// Provides: {"aarch64"}
// Dependencies: {}
# [cfg (all (any (target_arch = "aarch64" , target_arch = "arm64ec") , any (target_feature = "lsfe" , portable_atomic_target_feature = "lsfe") , target_feature = "neon" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) ,))] mod aarch64 ;
};
}
