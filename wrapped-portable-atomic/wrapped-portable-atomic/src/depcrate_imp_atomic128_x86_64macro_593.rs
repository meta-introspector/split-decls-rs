// Generated macro for macro_593 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_593 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_593"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_umin (dst : * mut u128 , val : u128) -> u128 ; cmpxchg16b = atomic_umin_cmpxchg16b ; fallback = atomic_umin_seqcst ; }
};
}
