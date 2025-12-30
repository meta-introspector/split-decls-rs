// Generated macro for atomic_load (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_load {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_load"}
// Dependencies: {}
# [inline] unsafe fn atomic_load (src : * mut u128 , _order : Ordering) -> u128 { # [cfg (all (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") , any (portable_atomic_no_outline_atomics , target_env = "sgx" , not (target_feature = "sse")) ,))] unsafe { atomic_load_cmpxchg16b (src) } # [cfg (not (all (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") , any (portable_atomic_no_outline_atomics , target_env = "sgx" , not (target_feature = "sse")) ,)))] unsafe { ifunc ! (unsafe fn (src : * mut u128) -> u128 { load_store_detect ! { vmovdqa = atomic_load_vmovdqa cmpxchg16b = atomic_load_cmpxchg16b fallback = atomic_load_seqcst } }) } }
};
}
