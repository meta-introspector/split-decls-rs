// Generated macro for macro_451 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_451 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_451"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_sub (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_sub_pwr8 ; non_seqcst_fallback = atomic_sub_non_seqcst ; seqcst_fallback = atomic_sub_seqcst ; }
};
}
