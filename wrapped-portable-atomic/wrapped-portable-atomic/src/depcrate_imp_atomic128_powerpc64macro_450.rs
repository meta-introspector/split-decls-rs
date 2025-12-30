// Generated macro for macro_450 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_450 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_450"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_add (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_add_pwr8 ; non_seqcst_fallback = atomic_add_non_seqcst ; seqcst_fallback = atomic_add_seqcst ; }
};
}
