// Generated macro for PFN_D3D12_CREATE_DEVICE (type)
macro_rules! Depcrate_delegate_cpp_refPFN_D3D12_CREATE_DEVICE {
() => {
// Module: crate::delegate_cpp_ref
// Provides: {"PFN_D3D12_CREATE_DEVICE"}
// Dependencies: {}
pub type PFN_D3D12_CREATE_DEVICE = Option < unsafe extern "system" fn (param0 : windows_core :: Ref < windows_core :: IUnknown > , param1 : D3D_FEATURE_LEVEL , param2 : * const windows_core :: GUID , param3 : * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , > ;
};
}
