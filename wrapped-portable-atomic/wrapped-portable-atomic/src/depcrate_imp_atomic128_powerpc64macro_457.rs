// Generated macro for macro_457 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_457 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_457"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umax (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_umax_pwr8 ; non_seqcst_fallback = atomic_umax_non_seqcst ; seqcst_fallback = atomic_umax_seqcst ; }
};
}
