// Generated macro for macro_590 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_590 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_590"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_max (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_max_cmpxchg16b ; fallback = atomic_max_seqcst ; }
};
}
