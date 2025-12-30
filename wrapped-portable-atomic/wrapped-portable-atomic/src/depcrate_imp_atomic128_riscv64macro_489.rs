// Generated macro for macro_489 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_489 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_489"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_add (dst : * mut u128 , val : u128) -> u128 { | x | x . wrapping_add (val) } zacas = atomic_add_zacas ; non_seqcst_fallback = atomic_add_non_seqcst ; seqcst_fallback = atomic_add_seqcst ; }
};
}
