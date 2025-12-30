// Generated macro for macro_491 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_491 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_491"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_and (dst : * mut u128 , val : u128) -> u128 { | x | x & val } zacas = atomic_and_zacas ; non_seqcst_fallback = atomic_and_non_seqcst ; seqcst_fallback = atomic_and_seqcst ; }
};
}
