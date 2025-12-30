// Generated macro for ID3D12Object_Vtbl (struct)
macro_rules! Depcrate_struct_with_cpp_interfaceID3D12Object_Vtbl {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"ID3D12Object_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ID3D12Object_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetPrivateData : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut u32 , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetPrivateData : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , u32 , * const core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetPrivateDataInterface : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetName : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR ,) -> windows_core :: HRESULT , }
};
}
