// Generated macro for macro_591 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_591 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_591"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umax (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_umax_cmpxchg16b ; fallback = atomic_umax_seqcst ; }
};
}
