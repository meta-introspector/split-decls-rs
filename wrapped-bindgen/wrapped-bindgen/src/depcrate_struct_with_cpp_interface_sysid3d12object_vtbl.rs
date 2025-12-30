// Generated macro for ID3D12Object_Vtbl (struct)
macro_rules! Depcrate_struct_with_cpp_interface_sysID3D12Object_Vtbl {
() => {
// Module: crate::struct_with_cpp_interface_sys
// Provides: {"ID3D12Object_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct ID3D12Object_Vtbl { pub base__ : windows_sys :: core :: IUnknown_Vtbl , pub GetPrivateData : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_sys :: core :: GUID , * mut u32 , * mut core :: ffi :: c_void ,) -> windows_sys :: core :: HRESULT , pub SetPrivateData : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_sys :: core :: GUID , u32 , * const core :: ffi :: c_void ,) -> windows_sys :: core :: HRESULT , pub SetPrivateDataInterface : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_sys :: core :: GUID , * mut core :: ffi :: c_void ,) -> windows_sys :: core :: HRESULT , pub SetName : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_sys :: core :: PCWSTR ,) -> windows_sys :: core :: HRESULT , }
};
}
