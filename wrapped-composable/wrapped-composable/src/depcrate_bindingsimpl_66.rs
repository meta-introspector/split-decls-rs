// Generated macro for impl_66 (impl)
macro_rules! Depcrate_bindingsimpl_66 {
() => {
// Module: crate::bindings
// Provides: {"impl_66"}
// Dependencies: {}
impl Visual { pub fn Compositor (& self) -> windows_core :: Result < Compositor > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Compositor) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
