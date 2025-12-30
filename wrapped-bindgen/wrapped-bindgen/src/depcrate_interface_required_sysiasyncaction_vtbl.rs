// Generated macro for IAsyncAction_Vtbl (struct)
macro_rules! Depcrate_interface_required_sysIAsyncAction_Vtbl {
() => {
// Module: crate::interface_required_sys
// Provides: {"IAsyncAction_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IAsyncAction_Vtbl { pub base__ : windows_sys :: core :: IInspectable_Vtbl , SetCompleted : usize , Completed : usize , pub GetResults : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_sys :: core :: HRESULT , }
};
}
