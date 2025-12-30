// Generated macro for IInspectable_Vtbl (struct)
macro_rules! Depcrate_inspectableIInspectable_Vtbl {
() => {
// Module: crate::inspectable
// Provides: {"IInspectable_Vtbl"}
// Dependencies: {}
# [doc (hidden)] # [repr (C)] pub struct IInspectable_Vtbl { pub base : IUnknown_Vtbl , pub GetIids : unsafe extern "system" fn (this : * mut c_void , count : * mut u32 , values : * mut * mut GUID ,) -> HRESULT , pub GetRuntimeClassName : unsafe extern "system" fn (this : * mut c_void , value : * mut * mut c_void) -> HRESULT , pub GetTrustLevel : unsafe extern "system" fn (this : * mut c_void , value : * mut i32) -> HRESULT , }
};
}
