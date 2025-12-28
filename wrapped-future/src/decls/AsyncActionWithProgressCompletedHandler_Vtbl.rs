macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! AsyncActionWithProgressCompletedHandler_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct AsyncActionWithProgressCompletedHandler_Vtbl < TProgress > where TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , TProgress : core :: marker :: PhantomData < TProgress > , }
    };
}

AsyncActionWithProgressCompletedHandler_Vtbl!();