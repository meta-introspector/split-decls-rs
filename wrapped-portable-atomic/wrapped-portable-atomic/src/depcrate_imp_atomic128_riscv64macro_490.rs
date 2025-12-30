// Generated macro for macro_490 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_490 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_490"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_sub (dst : * mut u128 , val : u128) -> u128 { | x | x . wrapping_sub (val) } zacas = atomic_sub_zacas ; non_seqcst_fallback = atomic_sub_non_seqcst ; seqcst_fallback = atomic_sub_seqcst ; }
};
}
