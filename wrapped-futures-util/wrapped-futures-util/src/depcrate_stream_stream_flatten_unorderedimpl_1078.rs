// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1078 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1078"}
// Dependencies: {}
impl ArcWake for WrappedWaker { fn wake_by_ref (self_arc : & Arc < Self >) { if let Some ((_ , state_bomb)) = self_arc . start_waking () { let waker_opt = unsafe { self_arc . inner_waker . get () . as_ref () . unwrap () } ; if let Some (inner_waker) = waker_opt . clone () { drop (state_bomb) ; inner_waker . wake () ; } } } }
};
}
