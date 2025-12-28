macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! AsyncActionCompletedHandler_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct AsyncActionCompletedHandler_Vtbl { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , asyncstatus : AsyncStatus ,) -> windows_core :: HRESULT , }
    };
}

AsyncActionCompletedHandler_Vtbl!();