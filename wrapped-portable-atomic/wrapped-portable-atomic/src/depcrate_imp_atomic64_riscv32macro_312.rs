// Generated macro for macro_312 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_312 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_312"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_and (dst : * mut u64 , val : u64) -> u64 { | x | x & val } zacas = atomic_and_zacas ; non_seqcst_fallback = atomic_and_non_seqcst ; seqcst_fallback = atomic_and_seqcst ; }
};
}
