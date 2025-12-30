// Generated macro for macro_455 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_455 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_455"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_xor (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_xor_pwr8 ; non_seqcst_fallback = atomic_xor_non_seqcst ; seqcst_fallback = atomic_xor_seqcst ; }
};
}
