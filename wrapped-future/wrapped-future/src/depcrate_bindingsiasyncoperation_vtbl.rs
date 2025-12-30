// Generated macro for IAsyncOperation_Vtbl (struct)
macro_rules! Depcrate_bindingsIAsyncOperation_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"IAsyncOperation_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncOperation_Vtbl < TResult > where TResult : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , pub SetCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub Completed : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut * mut core :: ffi :: c_void ,) -> windows_core :: HRESULT , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < TResult > ,) -> windows_core :: HRESULT , TResult : core :: marker :: PhantomData < TResult > , }
};
}
