// Generated macro for macro_493 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_493 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_493"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_or (dst : * mut u128 , val : u128) -> u128 { | x | x | val } zacas = atomic_or_zacas ; non_seqcst_fallback = atomic_or_non_seqcst ; seqcst_fallback = atomic_or_seqcst ; }
};
}
