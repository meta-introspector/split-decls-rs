// Generated macro for macro_313 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_313 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_313"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_nand (dst : * mut u64 , val : u64) -> u64 { | x | ! (x & val) } zacas = atomic_nand_zacas ; non_seqcst_fallback = atomic_nand_non_seqcst ; seqcst_fallback = atomic_nand_seqcst ; }
};
}
