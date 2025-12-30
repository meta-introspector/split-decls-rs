// Generated macro for IComposableFactory_Vtbl (struct)
macro_rules! Depcrate_bindingsIComposableFactory_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IComposableFactory_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IComposableFactory_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub CreateInstance : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub WithValue : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
