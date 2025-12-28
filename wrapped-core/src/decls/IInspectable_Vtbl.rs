macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
        HRESULT!();
        GUID!();
    };
}

macro_rules! IInspectable_Vtbl {
    () => {
        deps!();
        # [doc (hidden)] # [repr (C)] pub struct IInspectable_Vtbl { pub base : IUnknown_Vtbl , pub GetIids : unsafe extern "system" fn (this : * mut c_void , count : * mut u32 , values : * mut * mut GUID ,) -> HRESULT , pub GetRuntimeClassName : unsafe extern "system" fn (this : * mut c_void , value : * mut * mut c_void) -> HRESULT , pub GetTrustLevel : unsafe extern "system" fn (this : * mut c_void , value : * mut i32) -> HRESULT , }
    };
}

IInspectable_Vtbl!()