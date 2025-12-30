// Generated macro for macro_498 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_498 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_498"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umin (dst : * mut u128 , val : u128) -> u128 { | x | core :: cmp :: min (x , val) } zacas = atomic_umin_zacas ; non_seqcst_fallback = atomic_umin_non_seqcst ; seqcst_fallback = atomic_umin_seqcst ; }
};
}
