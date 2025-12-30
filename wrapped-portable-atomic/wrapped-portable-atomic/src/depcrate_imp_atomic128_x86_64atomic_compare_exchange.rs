// Generated macro for atomic_compare_exchange (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_compare_exchange {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_compare_exchange"}
// Dependencies: {}
# [inline] unsafe fn atomic_compare_exchange (dst : * mut u128 , old : u128 , new : u128 , _success : Ordering , _failure : Ordering ,) -> Result < u128 , u128 > { # [cfg (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b"))] let (prev , ok) = unsafe { cmpxchg16b (dst , old , new) } ; # [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b")))] let (prev , ok) = unsafe { ifunc ! (unsafe fn (dst : * mut u128 , old : u128 , new : u128) -> (u128 , bool) { if detect :: detect () . cmpxchg16b () { cmpxchg16b } else { fallback :: atomic_compare_exchange_seqcst } }) } ; if ok { Ok (prev) } else { Err (prev) } }
};
}
