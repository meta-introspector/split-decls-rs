// Generated macro for IE_Vtbl (struct)
macro_rules! Depcrate_bindingsIE_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IE_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IE_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub MethodOne : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut i32) -> windows_core :: HRESULT , pub MethodTwo : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut i32) -> windows_core :: HRESULT , }
};
}
