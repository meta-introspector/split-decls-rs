// Generated macro for ICompositor_Vtbl (struct)
macro_rules! Depcrate_bindingsICompositor_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"ICompositor_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct ICompositor_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , pub CreateSpriteVisual : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub CreateContainerVisual : unsafe extern "system" fn (* mut core :: ffi :: c_void , i32 , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , }
};
}
