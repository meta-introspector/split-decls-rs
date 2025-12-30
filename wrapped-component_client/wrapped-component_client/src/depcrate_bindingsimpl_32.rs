// Generated macro for impl_32 (impl)
macro_rules! Depcrate_bindingsimpl_32 {
() => {
// Module: crate::bindings
// Provides: {"impl_32"}
// Dependencies: {}
impl IThing { pub fn Method (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Method) (windows_core :: Interface :: as_raw (this)) . ok () } } }
};
}
