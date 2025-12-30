// Generated macro for macro_271 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_271 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_271"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_add (dst : * mut u64 , val : u64) -> u64 { | x | x . wrapping_add (val) } fallback = atomic_add_seqcst }
};
}
