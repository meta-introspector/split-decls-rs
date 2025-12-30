// Generated macro for macro_282 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_282 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_282"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_neg (dst : * mut u64) -> u64 { | x | x . wrapping_neg () } fallback = atomic_neg_seqcst }
};
}
