// Generated macro for macro_461 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_461 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_461"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_neg (dst : * mut u128) -> u128 ; pwr8 = atomic_neg_pwr8 ; non_seqcst_fallback = atomic_neg_non_seqcst ; seqcst_fallback = atomic_neg_seqcst ; }
};
}
