// Generated macro for atomic_load (function)
macro_rules! Depcrate_imp_atomic64_riscv32atomic_load {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"atomic_load"}
// Dependencies: {}
# [cfg (not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")))] # [inline] unsafe fn atomic_load (src : * mut u64 , order : Ordering) -> u64 { fn_alias ! { # [inline (never)] unsafe fn (src : * mut u64) -> u64 ; atomic_load_zacas_relaxed = atomic_load_zacas (Ordering :: Relaxed) ; atomic_load_zacas_acquire = atomic_load_zacas (Ordering :: Acquire) ; atomic_load_zacas_seqcst = atomic_load_zacas (Ordering :: SeqCst) ; } unsafe { match order { Ordering :: Relaxed => { ifunc ! (unsafe fn (src : * mut u64) -> u64 { if detect :: detect () . zacas () { atomic_load_zacas_relaxed } else { fallback :: atomic_load_non_seqcst } }) } Ordering :: Acquire => { ifunc ! (unsafe fn (src : * mut u64) -> u64 { if detect :: detect () . zacas () { atomic_load_zacas_acquire } else { fallback :: atomic_load_non_seqcst } }) } Ordering :: SeqCst => { ifunc ! (unsafe fn (src : * mut u64) -> u64 { if detect :: detect () . zacas () { atomic_load_zacas_seqcst } else { fallback :: atomic_load_seqcst } }) } _ => unreachable ! () , } } }
};
}
