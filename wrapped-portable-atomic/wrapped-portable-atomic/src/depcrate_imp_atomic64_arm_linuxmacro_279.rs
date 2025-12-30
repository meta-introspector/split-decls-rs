// Generated macro for macro_279 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_279 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_279"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_min (dst : * mut u64 , val : u64) -> u64 { | x | { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] { core :: cmp :: min (x as i64 , val as i64) as u64 } } } fallback = atomic_min_seqcst }
};
}
