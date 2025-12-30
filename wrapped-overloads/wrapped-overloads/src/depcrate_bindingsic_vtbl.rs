// Generated macro for IC_Vtbl (struct)
macro_rules! Depcrate_bindingsIC_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IC_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IC_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Method : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut i32) -> windows_core :: HRESULT , pub Method2 : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut i32) -> windows_core :: HRESULT , }
};
}
