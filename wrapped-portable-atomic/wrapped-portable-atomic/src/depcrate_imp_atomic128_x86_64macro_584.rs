// Generated macro for macro_584 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_584 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_584"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_add (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_add_cmpxchg16b ; fallback = atomic_add_seqcst ; }
};
}
