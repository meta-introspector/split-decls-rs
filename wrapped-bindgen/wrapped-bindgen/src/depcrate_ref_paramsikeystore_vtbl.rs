// Generated macro for IKeyStore_Vtbl (struct)
macro_rules! Depcrate_ref_paramsIKeyStore_Vtbl {
() => {
// Module: crate::ref_params
// Provides: {"IKeyStore_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IKeyStore_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetKey : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetKey : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub GetKeyValue : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetKeyValue : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub ClearKeys : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
