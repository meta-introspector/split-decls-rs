// Generated macro for impl_40 (impl)
macro_rules! Depcrate_bindingsimpl_40 {
() => {
// Module: crate::bindings
// Provides: {"impl_40"}
// Dependencies: {}
impl SpriteVisual { pub fn Children (& self) -> i32 { let this = & windows_core :: Interface :: cast :: < IContainerVisual > (self) . unwrap () ; unsafe { let mut result__ = core :: mem :: zeroed () ; let hresult__ = (windows_core :: Interface :: vtable (this) . Children) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) ; debug_assert ! (hresult__ . 0 == 0) ; result__ } } pub fn Brush (& self) -> i32 { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; let hresult__ = (windows_core :: Interface :: vtable (this) . Brush) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) ; debug_assert ! (hresult__ . 0 == 0) ; result__ } } pub fn Compositor (& self) -> windows_core :: Result < Compositor > { let this = & windows_core :: Interface :: cast :: < IVisual > (self) ? ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Compositor) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
