// Generated macro for macro_274 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_274 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_274"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_nand (dst : * mut u64 , val : u64) -> u64 { | x | ! (x & val) } fallback = atomic_nand_seqcst }
};
}
