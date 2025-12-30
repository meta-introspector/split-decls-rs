// Generated macro for impl_305 (impl)
macro_rules! Depcrate_future_try_maybe_doneimpl_305 {
() => {
// Module: crate::future::try_maybe_done
// Provides: {"impl_305"}
// Dependencies: {}
impl < Fut : TryFuture > FusedFuture for TryMaybeDone < Fut > { fn is_terminated (& self) -> bool { match self { Self :: Future (_) => false , Self :: Done (_) | Self :: Gone => true , } } }
};
}
