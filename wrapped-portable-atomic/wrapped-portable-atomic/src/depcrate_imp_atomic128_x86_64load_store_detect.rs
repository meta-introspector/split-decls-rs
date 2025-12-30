// Generated macro for load_store_detect (macro)
macro_rules! Depcrate_imp_atomic128_x86_64load_store_detect {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"load_store_detect"}
// Dependencies: {}
# [cfg (not (all (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b") , any (portable_atomic_no_outline_atomics , target_env = "sgx" , not (target_feature = "sse")) ,)))] macro_rules ! load_store_detect { (vmovdqa = $ vmovdqa : ident cmpxchg16b = $ cmpxchg16b : ident fallback = $ fallback : ident) => { { let cpuid = detect :: detect () ; # [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b" ,)))] { if cpuid . cmpxchg16b () { # [cfg (target_feature = "sse")] { if cpuid . vmovdqa_atomic () { $ vmovdqa } else { $ cmpxchg16b } } # [cfg (not (target_feature = "sse"))] { $ cmpxchg16b } } else { fallback ::$ fallback } } # [cfg (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b"))] { if cpuid . vmovdqa_atomic () { $ vmovdqa } else { $ cmpxchg16b } } } } ; }
};
}
