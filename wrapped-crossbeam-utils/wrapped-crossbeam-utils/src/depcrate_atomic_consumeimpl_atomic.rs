// Generated macro for impl_atomic (macro)
macro_rules! Depcrate_atomic_consumeimpl_atomic {
() => {
// Module: crate::atomic::consume
// Provides: {"impl_atomic"}
// Dependencies: {}
macro_rules ! impl_atomic { ($ atomic : ident , $ val : ty) => { # [cfg (not (crossbeam_no_atomic))] impl AtomicConsume for core :: sync :: atomic ::$ atomic { type Val = $ val ; impl_consume ! () ; } # [cfg (crossbeam_loom)] impl AtomicConsume for loom :: sync :: atomic ::$ atomic { type Val = $ val ; impl_consume ! () ; } } ; }
};
}
