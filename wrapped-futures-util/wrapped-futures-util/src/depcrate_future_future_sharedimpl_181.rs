// Generated macro for impl_181 (impl)
macro_rules! Depcrate_future_future_sharedimpl_181 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_181"}
// Dependencies: {}
impl < Fut > FusedFuture for Shared < Fut > where Fut : Future , Fut :: Output : Clone , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
};
}
