macro_rules! deps {
    () => {
        BSTR!();
        HRESULT!();
        IUnknown_Vtbl!();
    };
}

macro_rules! IRestrictedErrorInfo_Vtbl {
    () => {
        deps!();
        # [repr (C)] pub struct IRestrictedErrorInfo_Vtbl { pub base__ : IUnknown_Vtbl , pub GetErrorDetails : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR , * mut HRESULT , * mut BSTR , * mut BSTR ,) -> HRESULT , pub GetReference : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR) -> HRESULT , }
    };
}

IRestrictedErrorInfo_Vtbl!()