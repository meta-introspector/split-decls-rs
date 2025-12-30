// Generated macro for AsyncActionWithProgressCompletedHandler_Vtbl (struct)
macro_rules! Depcrate_bindingsAsyncActionWithProgressCompletedHandler_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionWithProgressCompletedHandler_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct AsyncActionWithProgressCompletedHandler_Vtbl < TProgress > where TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , TProgress : core :: marker :: PhantomData < TProgress > , }
};
}
