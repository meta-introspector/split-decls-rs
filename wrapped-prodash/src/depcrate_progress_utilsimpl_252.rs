// Generated macro for impl_252 (impl)
macro_rules! Depcrate_progress_utilsimpl_252 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_252"}
// Dependencies: {}
impl < T : NestedProgress > ThroughputOnDrop < T > { # [doc = " Create a new instance by providing the `inner` [`NestedProgress`] implementation."] pub fn new (inner : T) -> Self { ThroughputOnDrop (inner , Instant :: now ()) } }
};
}
