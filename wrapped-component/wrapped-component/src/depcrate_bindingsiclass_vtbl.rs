// Generated macro for IClass_Vtbl (struct)
macro_rules! Depcrate_bindingsIClass_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IClass_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IClass_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub Property : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut i32) -> windows_core :: HRESULT , pub SetProperty : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32) -> windows_core :: HRESULT , pub Flags : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut Flags) -> windows_core :: HRESULT , pub Int32Array : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , * const i32 , u32 , * mut i32 , * mut u32 , * mut * mut i32 , * mut u32 , * mut * mut i32 ,) -> windows_core :: HRESULT , pub StringArray : unsafe extern "system" fn (* mut core :: ffi :: c_void , u32 , * const windows_core :: HSTRING , u32 , * mut windows_core :: HSTRING , * mut u32 , * mut * mut windows_core :: HSTRING , * mut u32 , * mut * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Input : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
