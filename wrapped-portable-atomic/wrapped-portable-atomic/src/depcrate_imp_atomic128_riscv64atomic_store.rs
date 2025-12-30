// Generated macro for atomic_store (function)
macro_rules! Depcrate_imp_atomic128_riscv64atomic_store {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"atomic_store"}
// Dependencies: {}
# [inline] unsafe fn atomic_store (dst : * mut u128 , val : u128 , order : Ordering) { unsafe { atomic_swap (dst , val , order) ; } }
};
}
