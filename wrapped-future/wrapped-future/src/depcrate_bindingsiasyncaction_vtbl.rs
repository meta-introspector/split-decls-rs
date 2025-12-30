// Generated macro for IAsyncAction_Vtbl (struct)
macro_rules! Depcrate_bindingsIAsyncAction_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncAction_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncAction_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub SetCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Completed : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
