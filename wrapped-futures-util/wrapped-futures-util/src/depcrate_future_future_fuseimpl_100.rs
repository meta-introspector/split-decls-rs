// Generated macro for impl_100 (impl)
macro_rules! Depcrate_future_future_fuseimpl_100 {
() => {
// Module: crate::future::future::fuse
// Provides: {"impl_100"}
// Dependencies: {}
impl < Fut : Future > FusedFuture for Fuse < Fut > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
