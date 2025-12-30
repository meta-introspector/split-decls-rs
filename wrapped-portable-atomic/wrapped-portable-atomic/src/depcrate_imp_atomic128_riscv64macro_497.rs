// Generated macro for macro_497 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_497 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_497"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_min (dst : * mut u128 , val : u128) -> u128 { | x | { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] { core :: cmp :: min (x as i128 , val as i128) as u128 } } } zacas = atomic_min_zacas ; non_seqcst_fallback = atomic_min_non_seqcst ; seqcst_fallback = atomic_min_seqcst ; }
};
}
