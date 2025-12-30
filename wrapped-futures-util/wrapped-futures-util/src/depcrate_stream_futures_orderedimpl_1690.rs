// Generated macro for impl_1690 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1690 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1690"}
// Dependencies: {}
impl < Fut : Future > FusedStream for FuturesOrdered < Fut > { fn is_terminated (& self) -> bool { self . in_progress_queue . is_terminated () && self . queued_outputs . is_empty () } }
};
}
