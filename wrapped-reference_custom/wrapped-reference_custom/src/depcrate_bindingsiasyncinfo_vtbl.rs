// Generated macro for IAsyncInfo_Vtbl (struct)
macro_rules! Depcrate_bindingsIAsyncInfo_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncInfo_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncInfo_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Id : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , Status : usize , pub ErrorCode : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: HRESULT ,) -> windows_core :: HRESULT , pub Cancel : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , pub Close : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
