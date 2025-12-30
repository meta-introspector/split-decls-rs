// Generated macro for IStringable_Vtbl (struct)
macro_rules! Depcrate_interface_sysIStringable_Vtbl {
() => {
// Module: crate::interface_sys
// Provides: {"IStringable_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IStringable_Vtbl { pub base__ : windows_sys :: core :: IInspectable_Vtbl , pub ToString : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_sys :: core :: HSTRING ,) -> windows_sys :: core :: HRESULT , }
};
}
