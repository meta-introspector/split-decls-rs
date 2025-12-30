// Generated macro for macro_453 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_453 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_453"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_nand (dst : * mut u128 , val : u128) -> u128 ; pwr8 = atomic_nand_pwr8 ; non_seqcst_fallback = atomic_nand_non_seqcst ; seqcst_fallback = atomic_nand_seqcst ; }
};
}
