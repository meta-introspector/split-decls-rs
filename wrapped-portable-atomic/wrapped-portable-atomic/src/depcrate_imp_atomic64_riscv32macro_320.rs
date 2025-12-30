// Generated macro for macro_320 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_320 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_320"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_not (dst : * mut u64) -> u64 { | x | ! x } zacas = atomic_not_zacas ; non_seqcst_fallback = atomic_not_non_seqcst ; seqcst_fallback = atomic_not_seqcst ; }
};
}
