// Generated macro for atomic_store_cmpxchg16b (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_store_cmpxchg16b {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_store_cmpxchg16b"}
// Dependencies: {}
# [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] # [inline] unsafe fn atomic_store_cmpxchg16b (dst : * mut u128 , val : u128) { unsafe { atomic_swap_cmpxchg16b (dst , val , Ordering :: SeqCst) ; } }
};
}
