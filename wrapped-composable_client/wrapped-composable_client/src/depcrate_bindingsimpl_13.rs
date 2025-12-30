// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bindingsimpl_13 {
() => {
// Module: crate::bindings
// Provides: {"impl_13"}
// Dependencies: {}
impl ContainerVisual { pub fn Children (& self) -> i32 { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; let hresult__ = (windows_core :: Interface :: vtable (this) . Children) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) ; debug_assert ! (hresult__ . 0 == 0) ; result__ } } pub fn Compositor (& self) -> windows_core :: Result < Compositor > { let this = & windows_core :: Interface :: cast :: < IVisual > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Compositor) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
