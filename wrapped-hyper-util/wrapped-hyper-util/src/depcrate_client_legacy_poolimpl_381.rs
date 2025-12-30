// Generated macro for impl_381 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_381 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_381"}
// Dependencies: {}
impl < T > WeakOpt < T > { fn none () -> Self { WeakOpt (None) } fn downgrade (arc : & Arc < T >) -> Self { WeakOpt (Some (Arc :: downgrade (arc))) } fn upgrade (& self) -> Option < Arc < T > > { self . 0 . as_ref () . and_then (Weak :: upgrade) } }
};
}
