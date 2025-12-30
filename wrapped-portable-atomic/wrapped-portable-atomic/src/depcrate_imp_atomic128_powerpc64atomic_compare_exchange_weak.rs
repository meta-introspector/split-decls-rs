// Generated macro for atomic_compare_exchange_weak (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_compare_exchange_weak {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_compare_exchange_weak"}
// Dependencies: {}
# [cfg (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,))] # [inline] unsafe fn atomic_compare_exchange_weak (dst : * mut u128 , old : u128 , new : u128 , success : Ordering , failure : Ordering ,) -> Result < u128 , u128 > { let (prev , ok) = unsafe { atomic_compare_exchange_weak_pwr8 (dst , old , new , success , failure) } ; if ok { Ok (prev) } else { Err (prev) } }
};
}
