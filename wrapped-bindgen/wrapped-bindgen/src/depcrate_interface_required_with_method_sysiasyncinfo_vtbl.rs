// Generated macro for IAsyncInfo_Vtbl (struct)
macro_rules! Depcrate_interface_required_with_method_sysIAsyncInfo_Vtbl {
() => {
// Module: crate::interface_required_with_method_sys
// Provides: {"IAsyncInfo_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IAsyncInfo_Vtbl { pub base__ : windows_sys :: core :: IInspectable_Vtbl , pub Id : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_sys :: core :: HRESULT , pub Status : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut AsyncStatus ,) -> windows_sys :: core :: HRESULT , pub ErrorCode : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_sys :: core :: HRESULT ,) -> windows_sys :: core :: HRESULT , pub Cancel : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_sys :: core :: HRESULT , pub Close : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_sys :: core :: HRESULT , }
};
}
