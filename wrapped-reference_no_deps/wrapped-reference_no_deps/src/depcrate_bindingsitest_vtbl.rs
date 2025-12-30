// Generated macro for ITest_Vtbl (struct)
macro_rules! Depcrate_bindingsITest_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ITest_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Numerics : unsafe extern "system" fn (* mut core :: ffi :: c_void , Vector2) -> windows_core :: HRESULT , pub Collections : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Async : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Windows : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
