// Generated macro for macro_500 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_500 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_500"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_neg (dst : * mut u128) -> u128 { | x | x . wrapping_neg () } zacas = atomic_neg_zacas ; non_seqcst_fallback = atomic_neg_non_seqcst ; seqcst_fallback = atomic_neg_seqcst ; }
};
}
