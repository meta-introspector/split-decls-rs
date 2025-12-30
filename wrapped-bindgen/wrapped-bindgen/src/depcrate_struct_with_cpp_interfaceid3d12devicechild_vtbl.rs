// Generated macro for ID3D12DeviceChild_Vtbl (struct)
macro_rules! Depcrate_struct_with_cpp_interfaceID3D12DeviceChild_Vtbl {
() => {
// Module: crate::struct_with_cpp_interface
// Provides: {"ID3D12DeviceChild_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ID3D12DeviceChild_Vtbl { pub base__ : ID3D12Object_Vtbl , pub GetDevice : unsafe extern "system" fn (* mut core :: ffi :: c_void , * const windows_core :: GUID , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
