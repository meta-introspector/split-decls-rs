// Generated macro for macro_592 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_592 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_592"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_min (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_min_cmpxchg16b ; fallback = atomic_min_seqcst ; }
};
}
