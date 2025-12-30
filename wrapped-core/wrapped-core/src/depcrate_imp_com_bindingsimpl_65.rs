// Generated macro for impl_65 (impl)
macro_rules! Depcrate_imp_com_bindingsimpl_65 {
() => {
// Module: crate::imp::com_bindings
// Provides: {"impl_65"}
// Dependencies: {}
impl IWeakReferenceSource { pub unsafe fn GetWeakReference (& self) -> windows_core :: Result < IWeakReference > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetWeakReference) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
