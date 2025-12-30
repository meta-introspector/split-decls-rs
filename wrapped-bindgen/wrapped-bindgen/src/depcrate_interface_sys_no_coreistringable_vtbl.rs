// Generated macro for IStringable_Vtbl (struct)
macro_rules! Depcrate_interface_sys_no_coreIStringable_Vtbl {
() => {
// Module: crate::interface_sys_no_core
// Provides: {"IStringable_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IStringable_Vtbl { pub base__ : IInspectable_Vtbl , pub ToString : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut HSTRING) -> HRESULT , }
};
}
