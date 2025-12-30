// Generated macro for macro_495 (macro)
macro_rules! Depcrate_imp_atomic128_riscv64macro_495 {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"macro_495"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_max (dst : * mut u128 , val : u128) -> u128 { | x | { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] { core :: cmp :: max (x as i128 , val as i128) as u128 } } } zacas = atomic_max_zacas ; non_seqcst_fallback = atomic_max_non_seqcst ; seqcst_fallback = atomic_max_seqcst ; }
};
}
