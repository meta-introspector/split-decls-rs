// Generated macro for Callback_Vtbl (struct)
macro_rules! Depcrate_bindingsCallback_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"Callback_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct Callback_Vtbl { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , a : i32 , result__ : * mut i32 ,) -> windows_core :: HRESULT , }
};
}
