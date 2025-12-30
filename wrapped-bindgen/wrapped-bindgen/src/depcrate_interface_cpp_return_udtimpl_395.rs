// Generated macro for impl_395 (impl)
macro_rules! Depcrate_interface_cpp_return_udtimpl_395 {
() => {
// Module: crate::interface_cpp_return_udt
// Provides: {"impl_395"}
// Dependencies: {}
impl ID2D1Bitmap { pub unsafe fn GetSize (& self) -> D2D_SIZE_F { unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (self) . GetSize) (windows_core :: Interface :: as_raw (self) , & mut result__ ,) ; result__ } } pub unsafe fn GetDpi (& self , dpix : * mut f32 , dpiy : * mut f32) { unsafe { (windows_core :: Interface :: vtable (self) . GetDpi) (windows_core :: Interface :: as_raw (self) , dpix as _ , dpiy as _ ,) } } }
};
}
