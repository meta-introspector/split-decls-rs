// Generated macro for macro_317 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_317 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_317"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umax (dst : * mut u64 , val : u64) -> u64 { | x | core :: cmp :: max (x , val) } zacas = atomic_umax_zacas ; non_seqcst_fallback = atomic_umax_non_seqcst ; seqcst_fallback = atomic_umax_seqcst ; }
};
}
