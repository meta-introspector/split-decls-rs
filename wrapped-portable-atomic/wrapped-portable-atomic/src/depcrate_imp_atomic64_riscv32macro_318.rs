// Generated macro for macro_318 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_318 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_318"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_min (dst : * mut u64 , val : u64) -> u64 { | x | { # [allow (clippy :: cast_possible_wrap , clippy :: cast_sign_loss)] { core :: cmp :: min (x as i64 , val as i64) as u64 } } } zacas = atomic_min_zacas ; non_seqcst_fallback = atomic_min_non_seqcst ; seqcst_fallback = atomic_min_seqcst ; }
};
}
