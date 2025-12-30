// Generated macro for macro_309 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_309 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_309"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_swap (dst : * mut u64 , val : u64) -> u64 { | _x | val } zacas = atomic_swap_zacas ; non_seqcst_fallback = atomic_swap_non_seqcst ; seqcst_fallback = atomic_swap_seqcst ; }
};
}
