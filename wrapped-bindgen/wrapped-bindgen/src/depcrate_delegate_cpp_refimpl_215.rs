// Generated macro for impl_215 (impl)
macro_rules! Depcrate_delegate_cpp_refimpl_215 {
() => {
// Module: crate::delegate_cpp_ref
// Provides: {"impl_215"}
// Dependencies: {}
impl IActivationFactory { pub unsafe fn ActivateInstance (& self) -> windows_core :: Result < windows_core :: IInspectable > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . ActivateInstance) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
};
}
