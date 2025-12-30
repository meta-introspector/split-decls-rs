// Generated macro for macro_459 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_459 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_459"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umin (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_umin_pwr8 ; non_seqcst_fallback = atomic_umin_non_seqcst ; seqcst_fallback = atomic_umin_seqcst ; }
};
}
