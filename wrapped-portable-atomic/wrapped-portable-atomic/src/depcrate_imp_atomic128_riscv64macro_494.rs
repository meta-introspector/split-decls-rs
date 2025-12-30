// Generated macro for macro_494 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_494 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_494"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_xor (dst : * mut u128 , val : u128) -> u128 { | x | x ^ val } zacas = atomic_xor_zacas ; non_seqcst_fallback = atomic_xor_non_seqcst ; seqcst_fallback = atomic_xor_seqcst ; }
};
}
