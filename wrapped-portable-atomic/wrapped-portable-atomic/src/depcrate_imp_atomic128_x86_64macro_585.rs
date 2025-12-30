// Generated macro for macro_585 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_585 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_585"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_sub (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_sub_cmpxchg16b ; fallback = atomic_sub_seqcst ; }
};
}
