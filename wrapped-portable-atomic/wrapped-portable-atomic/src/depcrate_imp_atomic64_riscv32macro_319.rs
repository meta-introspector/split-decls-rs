// Generated macro for macro_319 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_319 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_319"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umin (dst : * mut u64 , val : u64) -> u64 { | x | core :: cmp :: min (x , val) } zacas = atomic_umin_zacas ; non_seqcst_fallback = atomic_umin_non_seqcst ; seqcst_fallback = atomic_umin_seqcst ; }
};
}
