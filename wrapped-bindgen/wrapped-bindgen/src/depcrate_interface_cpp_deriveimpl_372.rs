// Generated macro for impl_372 (impl)
macro_rules! Depcrate_interface_cpp_deriveimpl_372 {
() => {
// Module: crate::interface_cpp_derive
// Provides: {"impl_372"}
// Dependencies: {}
impl IPersist { pub unsafe fn GetClassID (& self) -> windows_core :: Result < windows_core :: GUID > { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetClassID) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) . map (| | result__) } } }
};
}
