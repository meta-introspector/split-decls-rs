// Generated macro for macro_586 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_586 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_586"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_and (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_and_cmpxchg16b ; fallback = atomic_and_seqcst ; }
};
}
