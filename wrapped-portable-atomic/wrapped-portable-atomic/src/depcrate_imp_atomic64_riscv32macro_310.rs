// Generated macro for macro_310 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_310 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_310"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_add (dst : * mut u64 , val : u64) -> u64 { | x | x . wrapping_add (val) } zacas = atomic_add_zacas ; non_seqcst_fallback = atomic_add_non_seqcst ; seqcst_fallback = atomic_add_seqcst ; }
};
}
