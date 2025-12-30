// Generated macro for impl_58 (impl)
macro_rules! Depcrate_imp_com_bindingsimpl_58 {
() => {
// Module: crate::imp::com_bindings
// Provides: {"impl_58"}
// Dependencies: {}
impl IWeakReference { pub unsafe fn Resolve < T > (& self) -> windows_core :: Result < T > where T : windows_core :: Interface , { let mut result__ = core :: ptr :: null_mut () ; unsafe { (windows_core :: Interface :: vtable (self) . Resolve) (windows_core :: Interface :: as_raw (self) , & T :: IID , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
