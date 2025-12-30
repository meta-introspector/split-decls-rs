// Generated macro for impl_15 (impl)
macro_rules! Depcrate_accessimpl_15 {
() => {
// Module: crate::access
// Provides: {"impl_15"}
// Dependencies: {}
impl < T > Access < T > for dyn DynAccess < T > + '_ + Sync + Send { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
};
}
