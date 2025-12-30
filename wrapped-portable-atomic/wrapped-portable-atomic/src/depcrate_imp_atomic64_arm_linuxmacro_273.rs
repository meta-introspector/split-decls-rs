// Generated macro for macro_273 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_273 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_273"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_and (dst : * mut u64 , val : u64) -> u64 { | x | x & val } fallback = atomic_and_seqcst }
};
}
