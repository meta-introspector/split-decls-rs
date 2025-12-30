// Generated macro for impl_291 (impl)
macro_rules! Depcrate_future_maybe_doneimpl_291 {
() => {
// Module: crate::future::maybe_done
// Provides: {"impl_291"}
// Dependencies: {}
impl < Fut : Future > FusedFuture for MaybeDone < Fut > { fn is_terminated (& self) -> bool { match self { Self :: Future (_) => false , Self :: Done (_) | Self :: Gone => true , } } }
};
}
