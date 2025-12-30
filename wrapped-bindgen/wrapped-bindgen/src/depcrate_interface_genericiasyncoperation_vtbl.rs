// Generated macro for IAsyncOperation_Vtbl (struct)
macro_rules! Depcrate_interface_genericIAsyncOperation_Vtbl {
() => {
// Module: crate::interface_generic
// Provides: {"IAsyncOperation_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncOperation_Vtbl < TResult > where TResult : windows_core :: RuntimeType + 'static , { pub base__ : windows_core :: IInspectable_Vtbl , SetCompleted : usize , Completed : usize , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: AbiType < TResult > ,) -> windows_core :: HRESULT , TResult : core :: marker :: PhantomData < TResult > , }
};
}
