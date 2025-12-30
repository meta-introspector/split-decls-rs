// Generated macro for atomic_store (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_store {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_store"}
// Dependencies: {}
# [inline] unsafe fn atomic_store (dst : * mut u128 , val : u128 , order : Ordering) { # [cfg (all (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") , any (portable_atomic_no_outline_atomics , target_env = "sgx" , not (target_feature = "sse")) ,))] unsafe { let _ = order ; atomic_store_cmpxchg16b (dst , val) ; } # [cfg (not (all (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") , any (portable_atomic_no_outline_atomics , target_env = "sgx" , not (target_feature = "sse")) ,)))] unsafe { # [cfg (target_feature = "sse")] fn_alias ! { # [target_feature (enable = "avx")] unsafe fn (dst : * mut u128 , val : u128) ; atomic_store_vmovdqa_non_seqcst = atomic_store_vmovdqa (Ordering :: Release) ; atomic_store_vmovdqa_seqcst = atomic_store_vmovdqa (Ordering :: SeqCst) ; } match order { Ordering :: Relaxed | Ordering :: Release => { ifunc ! (unsafe fn (dst : * mut u128 , val : u128) { load_store_detect ! { vmovdqa = atomic_store_vmovdqa_non_seqcst cmpxchg16b = atomic_store_cmpxchg16b fallback = atomic_store_non_seqcst } }) ; } Ordering :: SeqCst => { ifunc ! (unsafe fn (dst : * mut u128 , val : u128) { load_store_detect ! { vmovdqa = atomic_store_vmovdqa_seqcst cmpxchg16b = atomic_store_cmpxchg16b fallback = atomic_store_seqcst } }) ; } _ => unreachable ! () , } } }
};
}
