// Generated macro for macro_316 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_316 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_316"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_max (dst : * mut u64 , val : u64) -> u64 { | x | { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] { core :: cmp :: max (x as i64 , val as i64) as u64 } } } zacas = atomic_max_zacas ; non_seqcst_fallback = atomic_max_non_seqcst ; seqcst_fallback = atomic_max_seqcst ; }
};
}
