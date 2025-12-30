// Generated macro for macro_460 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_460 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_460"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_not (dst : * mut u128) -> u128 ; pwr8 = atomic_not_pwr8 ; non_seqcst_fallback = atomic_not_non_seqcst ; seqcst_fallback = atomic_not_seqcst ; }
};
}
