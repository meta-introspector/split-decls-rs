// Generated macro for atomic_load (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_load {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_load"}
// Dependencies: {}
# [cfg (not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)))] # [inline] unsafe fn atomic_load (src : * mut u128 , order : Ordering) -> u128 { fn_alias ! { # [inline (never)] unsafe fn (src : * mut u128) -> u128 ; atomic_load_pwr8_relaxed = atomic_load_pwr8 (Ordering :: Relaxed) ; atomic_load_pwr8_acquire = atomic_load_pwr8 (Ordering :: Acquire) ; atomic_load_pwr8_seqcst = atomic_load_pwr8 (Ordering :: SeqCst) ; } unsafe { match order { Ordering :: Relaxed => { ifunc ! (unsafe fn (src : * mut u128) -> u128 { if detect :: detect () . quadword_atomics () { atomic_load_pwr8_relaxed } else { fallback :: atomic_load_non_seqcst } }) } Ordering :: Acquire => { ifunc ! (unsafe fn (src : * mut u128) -> u128 { if detect :: detect () . quadword_atomics () { atomic_load_pwr8_acquire } else { fallback :: atomic_load_non_seqcst } }) } Ordering :: SeqCst => { ifunc ! (unsafe fn (src : * mut u128) -> u128 { if detect :: detect () . quadword_atomics () { atomic_load_pwr8_seqcst } else { fallback :: atomic_load_seqcst } }) } _ => unreachable ! () , } } }
};
}
