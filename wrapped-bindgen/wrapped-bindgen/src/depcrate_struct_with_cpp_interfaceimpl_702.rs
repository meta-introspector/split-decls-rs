// Generated macro for impl_702 (impl)
macro_rules! Depcrate_struct_with_cpp_interfaceimpl_702 {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"impl_702"}
// Dependencies: {}
impl ID3D12DeviceChild_Vtbl { pub const fn new < Identity : ID3D12DeviceChild_Impl , const OFFSET : isize > () -> Self { unsafe extern "system" fn GetDevice < Identity : ID3D12DeviceChild_Impl , const OFFSET : isize , > (this : * mut core :: ffi :: c_void , riid : * const windows_core :: GUID , ppvdevice : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT { unsafe { let this : & Identity = & * ((this as * const * const ()) . offset (OFFSET) as * const Identity) ; ID3D12DeviceChild_Impl :: GetDevice (this , core :: mem :: transmute_copy (& riid) , core :: mem :: transmute_copy (& ppvdevice) ,) . into () } } Self { base__ : ID3D12Object_Vtbl :: new :: < Identity , OFFSET > () , GetDevice : GetDevice :: < Identity , OFFSET > , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < ID3D12DeviceChild as windows_core :: Interface > :: IID || iid == & < ID3D12Object as windows_core :: Interface > :: IID } }
};
}
