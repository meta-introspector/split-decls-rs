// Generated macro for macro_311 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_311 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_311"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_sub (dst : * mut u64 , val : u64) -> u64 { | x | x . wrapping_sub (val) } zacas = atomic_sub_zacas ; non_seqcst_fallback = atomic_sub_non_seqcst ; seqcst_fallback = atomic_sub_seqcst ; }
};
}
