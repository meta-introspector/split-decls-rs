// Generated macro for atomic_update (function)
macro_rules! Depcrate_imp_atomic128_s390xatomic_update {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_update"}
// Dependencies: {}
# [cfg (not (any (target_feature = "load-store-on-cond" , portable_atomic_target_feature = "load-store-on-cond" ,)))] # [inline (always)] unsafe fn atomic_update < F > (dst : * mut u128 , order : Ordering , mut f : F) -> u128 where F : FnMut (u128) -> u128 , { unsafe { let mut prev = byte_wise_atomic_load (dst) ; loop { let next = f (prev) ; match atomic_compare_exchange_weak (dst , prev , next , order , Ordering :: Relaxed) { Ok (x) => return x , Err (x) => prev = x , } } } }
};
}
