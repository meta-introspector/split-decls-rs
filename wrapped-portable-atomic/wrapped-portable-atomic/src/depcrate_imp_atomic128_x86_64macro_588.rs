// Generated macro for macro_588 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_588 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_588"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_or (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_or_cmpxchg16b ; fallback = atomic_or_seqcst ; }
};
}
