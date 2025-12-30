// Generated macro for macro_281 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_281 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_281"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_not (dst : * mut u64) -> u64 { | x | ! x } fallback = atomic_not_seqcst }
};
}
