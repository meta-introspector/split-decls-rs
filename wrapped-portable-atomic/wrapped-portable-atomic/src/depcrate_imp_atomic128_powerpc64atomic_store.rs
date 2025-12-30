// Generated macro for atomic_store (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_store {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_store"}
// Dependencies: {}
# [cfg (not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)))] # [inline] unsafe fn atomic_store (dst : * mut u128 , val : u128 , order : Ordering) { fn_alias ! { # [inline (never)] unsafe fn (dst : * mut u128 , val : u128) ; atomic_store_pwr8_relaxed = atomic_store_pwr8 (Ordering :: Relaxed) ; atomic_store_pwr8_release = atomic_store_pwr8 (Ordering :: Release) ; atomic_store_pwr8_seqcst = atomic_store_pwr8 (Ordering :: SeqCst) ; } unsafe { match order { Ordering :: Relaxed => { ifunc ! (unsafe fn (dst : * mut u128 , val : u128) { if detect :: detect () . quadword_atomics () { atomic_store_pwr8_relaxed } else { fallback :: atomic_store_non_seqcst } }) ; } Ordering :: Release => { ifunc ! (unsafe fn (dst : * mut u128 , val : u128) { if detect :: detect () . quadword_atomics () { atomic_store_pwr8_release } else { fallback :: atomic_store_non_seqcst } }) ; } Ordering :: SeqCst => { ifunc ! (unsafe fn (dst : * mut u128 , val : u128) { if detect :: detect () . quadword_atomics () { atomic_store_pwr8_seqcst } else { fallback :: atomic_store_seqcst } }) ; } _ => unreachable ! () , } } }
};
}
