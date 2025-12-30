// Generated macro for AsyncOperationWithProgressCompletedHandler_Vtbl (struct)
macro_rules! Depcrate_bindingsAsyncOperationWithProgressCompletedHandler_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"AsyncOperationWithProgressCompletedHandler_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct AsyncOperationWithProgressCompletedHandler_Vtbl < TResult , TProgress > where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , TResult : core :: marker :: PhantomData < TResult > , TProgress : core :: marker :: PhantomData < TProgress > , }
};
}
