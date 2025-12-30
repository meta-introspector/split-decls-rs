// Generated macro for macro_496 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_496 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_496"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umax (dst : * mut u128 , val : u128) -> u128 { | x | core :: cmp :: max (x , val) } zacas = atomic_umax_zacas ; non_seqcst_fallback = atomic_umax_non_seqcst ; seqcst_fallback = atomic_umax_seqcst ; }
};
}
