// Generated macro for macro_315 (macro)
macro_rules! Depcrate_imp_atomic64_riscv32macro_315 {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"macro_315"}
// Dependencies: {}
select_atomic_rmw ! { unsafe fn atomic_xor (dst : * mut u64 , val : u64) -> u64 { | x | x ^ val } zacas = atomic_xor_zacas ; non_seqcst_fallback = atomic_xor_non_seqcst ; seqcst_fallback = atomic_xor_seqcst ; }
};
}
