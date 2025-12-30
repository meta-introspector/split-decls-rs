// Generated macro for macro_268 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_268 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_268"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_swap (dst : * mut u64 , val : u64) -> u64 { | _x | val } fallback = atomic_swap_seqcst }
};
}
