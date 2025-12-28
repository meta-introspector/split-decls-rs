macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! IAsyncInfo_Vtbl {
    () => {
        deps!();
        # [repr (C)] # [doc (hidden)] pub struct IAsyncInfo_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Id : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> windows_core :: HRESULT , pub Status : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut AsyncStatus ,) -> windows_core :: HRESULT , pub ErrorCode : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: HRESULT ,) -> windows_core :: HRESULT , pub Cancel : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , pub Close : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
    };
}

IAsyncInfo_Vtbl!()