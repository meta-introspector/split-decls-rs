// Generated macro for IRestrictedErrorInfo_Vtbl (struct)
macro_rules! Depcrate_bindingsIRestrictedErrorInfo_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IRestrictedErrorInfo_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IRestrictedErrorInfo_Vtbl { pub base__ : IUnknown_Vtbl , pub GetErrorDetails : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR , * mut HRESULT , * mut BSTR , * mut BSTR ,) -> HRESULT , pub GetReference : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR) -> HRESULT , }
};
}
