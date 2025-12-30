// Generated macro for impl_13 (impl)
macro_rules! Depcrate_accessimpl_13 {
() => {
// Module: crate::access
// Provides: {"impl_13"}
// Dependencies: {}
impl < T > Access < T > for dyn DynAccess < T > + '_ { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
};
}
