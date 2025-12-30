// Generated macro for macro_452 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_452 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_452"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_and (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_and_pwr8 ; non_seqcst_fallback = atomic_and_non_seqcst ; seqcst_fallback = atomic_and_seqcst ; }
};
}
