// Generated macro for macro_449 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_449 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_449"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_swap (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_swap_pwr8 ; non_seqcst_fallback = atomic_swap_non_seqcst ; seqcst_fallback = atomic_swap_seqcst ; }
};
}
