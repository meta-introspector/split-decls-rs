// Generated macro for impl_170 (impl)
macro_rules! Depcrate_future_future_sharedimpl_170 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_170"}
// Dependencies: {}
unsafe impl < Fut > Send for Inner < Fut > where Fut : Future + Send , Fut :: Output : Send + Sync , { }
};
}
