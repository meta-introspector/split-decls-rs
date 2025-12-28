macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! AsyncOperationWithProgressCompletedHandler_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct AsyncOperationWithProgressCompletedHandler_Vtbl < TResult , TProgress > where TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , TResult : core :: marker :: PhantomData < TResult > , TProgress : core :: marker :: PhantomData < TProgress > , }
    };
}

AsyncOperationWithProgressCompletedHandler_Vtbl!();