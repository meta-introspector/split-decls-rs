// Generated macro for macro_458 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_458 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_458"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_min (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_min_pwr8 ; non_seqcst_fallback = atomic_min_non_seqcst ; seqcst_fallback = atomic_min_seqcst ; }
};
}
