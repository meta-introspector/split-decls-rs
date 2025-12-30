// Generated macro for impl_478 (impl)
macro_rules! Depcrate_concurrency_syncimpl_478 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_478"}
// Dependencies: {}
impl < 'tcx > AllocExtra < 'tcx > { fn get_sync < T : 'static > (& self , offset : Size) -> Option < & T > { self . sync . get (& offset) . and_then (| s | s . downcast_ref :: < T > ()) } }
};
}
