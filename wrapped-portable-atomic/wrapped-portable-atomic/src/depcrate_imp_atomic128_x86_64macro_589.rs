// Generated macro for macro_589 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_589 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_589"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_xor (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_xor_cmpxchg16b ; fallback = atomic_xor_seqcst ; }
};
}
