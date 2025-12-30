// Generated macro for impl_243 (impl)
macro_rules! Depcrate_scoped_interfaceimpl_243 {
() => {
// Module: crate::scoped_interface
// Provides: {"impl_243"}
// Dependencies: {}
impl < T : Interface > core :: ops :: Deref for ScopedInterface < '_ , T > { type Target = T ; fn deref (& self) -> & T { & self . interface } }
};
}
