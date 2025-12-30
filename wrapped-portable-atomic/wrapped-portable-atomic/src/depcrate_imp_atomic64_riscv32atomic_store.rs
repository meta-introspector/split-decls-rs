// Generated macro for atomic_store (function)
macro_rules! Depcrate_imp_atomic64_riscv32atomic_store {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"atomic_store"}
// Dependencies: {}
# [inline] unsafe fn atomic_store (dst : * mut u64 , val : u64 , order : Ordering) { unsafe { atomic_swap (dst , val , order) ; } }
};
}
