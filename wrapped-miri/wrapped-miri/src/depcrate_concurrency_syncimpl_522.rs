// Generated macro for impl_522 (impl)
macro_rules! Depcrate_concurrency_syncimpl_522 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_522"}
// Dependencies: {}
impl < 'tcx > AllocExtra < 'tcx > { fn get_sync < T : 'static > (& self , offset : Size) -> Option < & T > { self . sync_objs . get (& offset) . and_then (| s | s . downcast_ref :: < T > ()) } }
};
}
