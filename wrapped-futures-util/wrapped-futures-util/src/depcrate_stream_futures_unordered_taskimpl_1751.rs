// Generated macro for impl_1751 (impl)
macro_rules! Depcrate_stream_futures_unordered_taskimpl_1751 {
() => {
// Module: crate::stream::futures_unordered::task
// Provides: {"impl_1751"}
// Dependencies: {}
impl < Fut > ArcWake for Task < Fut > { fn wake_by_ref (arc_self : & Arc < Self >) { let inner = match arc_self . ready_to_run_queue . upgrade () { Some (inner) => inner , None => return , } ; arc_self . woken . store (true , Relaxed) ; let prev = arc_self . queued . swap (true , SeqCst) ; if ! prev { inner . enqueue (Arc :: as_ptr (arc_self)) ; inner . waker . wake () ; } } }
};
}
