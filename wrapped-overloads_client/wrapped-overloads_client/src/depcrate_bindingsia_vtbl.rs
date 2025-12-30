// Generated macro for IA_Vtbl (struct)
macro_rules! Depcrate_bindingsIA_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IA_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IA_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Method : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut i32) -> windows_core :: HRESULT , pub Method2 : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut i32) -> windows_core :: HRESULT , }
};
}
