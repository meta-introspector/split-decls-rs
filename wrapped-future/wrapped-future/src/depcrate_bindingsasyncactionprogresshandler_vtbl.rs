// Generated macro for AsyncActionProgressHandler_Vtbl (struct)
macro_rules! Depcrate_bindingsAsyncActionProgressHandler_Vtbl {
() => {
// Module: crate::bindings
// Provides: {"AsyncActionProgressHandler_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct AsyncActionProgressHandler_Vtbl < TProgress > where TProgress : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , asyncinfo : * mut core :: ffi :: c_void , progressinfo : windows_core :: AbiType < TProgress > ,) -> windows_core :: HRESULT , TProgress : core :: marker :: PhantomData < TProgress > , }
};
}
