// Generated macro for macro_492 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_492 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_492"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_nand (dst : * mut u128 , val : u128) -> u128 { | x | ! (x & val) } zacas = atomic_nand_zacas ; non_seqcst_fallback = atomic_nand_non_seqcst ; seqcst_fallback = atomic_nand_seqcst ; }
};
}
