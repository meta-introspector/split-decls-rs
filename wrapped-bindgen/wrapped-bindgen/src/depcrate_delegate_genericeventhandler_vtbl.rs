// Generated macro for EventHandler_Vtbl (struct)
macro_rules! Depcrate_delegate_genericEventHandler_Vtbl {
() => {
// Module: crate::delegate_generic
// Provides: {"EventHandler_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct EventHandler_Vtbl < T > where T : windows_core :: RuntimeType + 'static , { base__ : windows_core :: IUnknown_Vtbl , Invoke : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , sender : * mut core :: ffi :: c_void , args : windows_core :: AbiType < T > ,) -> windows_core :: HRESULT , T : core :: marker :: PhantomData < T > , }
};
}
