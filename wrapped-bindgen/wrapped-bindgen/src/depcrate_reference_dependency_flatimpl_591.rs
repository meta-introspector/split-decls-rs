// Generated macro for impl_591 (impl)
macro_rules! Depcrate_reference_dependency_flatimpl_591 {
() => {
// Module: crate::reference_dependency_flat
// Provides: {"impl_591"}
// Dependencies: {}
impl IClosable { pub fn Close (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
