// Generated macro for macro_594 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_594 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_594"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_not (dst : * mut u128) -> u128 ; cmpxchg16b = atomic_not_cmpxchg16b ; fallback = atomic_not_seqcst ; }
};
}
