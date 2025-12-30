// Generated macro for ITest_Vtbl (struct)
macro_rules! Depcrate_bindingsITest_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ITest_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Input : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut i32 ,) -> windows_core :: HRESULT , pub Output : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Current : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut i32) -> windows_core :: HRESULT , pub SetCurrent : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32) -> windows_core :: HRESULT , }
};
}
