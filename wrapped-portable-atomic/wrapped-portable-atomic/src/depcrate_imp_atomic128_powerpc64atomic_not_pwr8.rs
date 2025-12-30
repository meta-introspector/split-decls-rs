// Generated macro for atomic_not_pwr8 (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_not_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_not_pwr8"}
// Dependencies: {}
# [inline] unsafe fn atomic_not_pwr8 (dst : * mut u128 , order : Ordering) -> u128 { unsafe { atomic_xor_pwr8 (dst , ! 0 , order) } }
};
}
