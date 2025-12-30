// Generated macro for IAsyncAction_Vtbl (struct)
macro_rules! Depcrate_interface_required_with_methodIAsyncAction_Vtbl {
() => {
// Module: crate::interface_required_with_method
// Provides: {"IAsyncAction_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IAsyncAction_Vtbl { pub base__ : windows_core :: IInspectable_Vtbl , SetCompleted : usize , Completed : usize , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , }
};
}
