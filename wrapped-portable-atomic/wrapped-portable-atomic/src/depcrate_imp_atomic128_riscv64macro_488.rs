// Generated macro for macro_488 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_488 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_488"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_swap (dst : * mut u128 , val : u128) -> u128 { | _x | val } zacas = atomic_swap_zacas ; non_seqcst_fallback = atomic_swap_non_seqcst ; seqcst_fallback = atomic_swap_seqcst ; }
};
}
