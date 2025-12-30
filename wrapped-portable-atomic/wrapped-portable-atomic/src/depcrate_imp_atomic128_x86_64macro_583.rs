// Generated macro for macro_583 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_583 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_583"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_swap (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_swap_cmpxchg16b ; fallback = atomic_swap_seqcst ; }
};
}
