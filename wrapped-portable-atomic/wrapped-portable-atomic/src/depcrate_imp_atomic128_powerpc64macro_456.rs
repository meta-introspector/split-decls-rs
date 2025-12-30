// Generated macro for macro_456 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_456 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_456"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_max (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_max_pwr8 ; non_seqcst_fallback = atomic_max_non_seqcst ; seqcst_fallback = atomic_max_seqcst ; }
};
}
