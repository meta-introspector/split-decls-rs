// Generated macro for IAsyncActionWithProgress_Vtbl (struct)
macro_rules! Depcrate_bindingsIAsyncActionWithProgress_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncActionWithProgress_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncActionWithProgress_Vtbl < TProgress > where TProgress : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub SetProgress : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Progress : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub SetCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Completed : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , TProgress : core :: marker :: PhantomData < TProgress > , }
};
}
