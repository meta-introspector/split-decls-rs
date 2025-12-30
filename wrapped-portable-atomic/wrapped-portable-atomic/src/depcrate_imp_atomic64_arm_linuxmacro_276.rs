// Generated macro for macro_276 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_276 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_276"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_xor (dst : * mut u64 , val : u64) -> u64 { | x | x ^ val } fallback = atomic_xor_seqcst }
};
}
