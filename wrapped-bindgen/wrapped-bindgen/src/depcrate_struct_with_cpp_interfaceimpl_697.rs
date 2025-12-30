// Generated macro for impl_697 (impl)
macro_rules! Depcrate_struct_with_cpp_interfaceimpl_697 {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"impl_697"}
// Dependencies: {}
impl ID3D12DeviceChild { pub unsafe fn GetDevice < T > (& self , result__ : * mut Option < T >) -> windows_core :: Result < () > where T : windows_core :: Interface , { unsafe { (windows_core :: Interface :: vtable (self) . GetDevice) (windows_core :: Interface :: as_raw (self) , & T :: IID , result__ as * mut _ as * mut _ ,) . ok () } } }
};
}
