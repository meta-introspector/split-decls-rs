// Generated macro for IClassStatics_Vtbl (struct)
macro_rules! Depcrate_bindingsIClassStatics_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IClassStatics_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IClassStatics_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub StaticSignal : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut i32) -> windows_core :: HRESULT , pub StaticEvent : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void , * mut i64 ,) -> windows_core :: HRESULT , pub RemoveStaticEvent : unsafe extern "system" fn (* mut core :: ffi :: c_void , i64) -> windows_core :: HRESULT , }
};
}
