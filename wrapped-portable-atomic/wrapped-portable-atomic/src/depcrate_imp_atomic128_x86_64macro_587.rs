// Generated macro for macro_587 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_587 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_587"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_nand (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_nand_cmpxchg16b ; fallback = atomic_nand_seqcst ; }
};
}
