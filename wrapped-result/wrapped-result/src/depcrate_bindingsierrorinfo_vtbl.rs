// Generated macro for IErrorInfo_Vtbl (struct)
macro_rules! Depcrate_bindingsIErrorInfo_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IErrorInfo_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IErrorInfo_Vtbl { pub base__ : IUnknown_Vtbl , pub GetGUID : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut GUID) -> HRESULT , pub GetSource : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR) -> HRESULT , pub GetDescription : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR) -> HRESULT , pub GetHelpFile : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut BSTR) -> HRESULT , pub GetHelpContext : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut u32) -> HRESULT , }
};
}
