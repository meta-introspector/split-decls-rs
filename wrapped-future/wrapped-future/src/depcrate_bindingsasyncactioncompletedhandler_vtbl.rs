// Generated macro for AsyncActionCompletedHandler_Vtbl (struct)
macro_rules! Depcrate_bindingsAsyncActionCompletedHandler_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionCompletedHandler_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct AsyncActionCompletedHandler_Vtbl { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , }
};
}
