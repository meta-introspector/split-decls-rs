// Generated macro for macro_499 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_499 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_499"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_not (dst : * mut u128) -> u128 { | x | ! x } zacas = atomic_not_zacas ; non_seqcst_fallback = atomic_not_non_seqcst ; seqcst_fallback = atomic_not_seqcst ; }
};
}
