// Generated macro for macro_321 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_321 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_321"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_neg (dst : * mut u64) -> u64 { | x | x . wrapping_neg () } zacas = atomic_neg_zacas ; non_seqcst_fallback = atomic_neg_non_seqcst ; seqcst_fallback = atomic_neg_seqcst ; }
};
}
