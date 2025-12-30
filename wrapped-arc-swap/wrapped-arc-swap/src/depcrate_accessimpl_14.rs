// Generated macro for impl_14 (impl)
macro_rules! Depcrate_accessimpl_14 {
() => {
// Module: crate::access
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > Access < T > for dyn DynAccess < T > + '_ + Send { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
};
}
