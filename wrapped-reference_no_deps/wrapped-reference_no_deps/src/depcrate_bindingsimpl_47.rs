// Generated macro for impl_47 (impl)
macro_rules! Depcrate_bindingsimpl_47 {
() => {
// Module: crate::bindings
// Provides: {"impl_47"}
// Dependencies: {}
impl IStringable { pub fn ToString (& self) -> windows_core :: Result < windows_core :: HSTRING > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . ToString) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | core :: mem :: transmute (result__)) } } }
};
}
