// Generated macro for macro_595 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_595 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_595"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_neg (dst : * mut u128) -> u128 ; cmpxchg16b = atomic_neg_cmpxchg16b ; fallback = atomic_neg_seqcst ; }
};
}
