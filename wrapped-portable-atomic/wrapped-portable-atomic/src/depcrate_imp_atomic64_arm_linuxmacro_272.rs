// Generated macro for macro_272 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_272 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_272"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_sub (dst : * mut u64 , val : u64) -> u64 { | x | x . wrapping_sub (val) } fallback = atomic_sub_seqcst }
};
}
