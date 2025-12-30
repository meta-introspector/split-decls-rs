// Generated macro for IClass_Vtbl (struct)
macro_rules! Depcrate_bindingsIClass_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IClass_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IClass_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Signal : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut i32) -> windows_core :: HRESULT , pub Event : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut i64 ,) -> windows_core :: HRESULT , pub RemoveEvent : unsafe extern "system" fn (* mut core :: ffi :: c_void , i64) -> windows_core :: HRESULT , }
};
}
