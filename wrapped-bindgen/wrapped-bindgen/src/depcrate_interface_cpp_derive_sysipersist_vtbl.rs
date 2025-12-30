// Generated macro for IPersist_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_derive_sysIPersist_Vtbl {
() => {
// Module: crate::interface_cpp_derive_sys
// Provides: {"IPersist_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IPersist_Vtbl { pub base__ : windows_sys :: core :: IUnknown_Vtbl , pub GetClassID : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_sys :: core :: GUID ,) -> windows_sys :: core :: HRESULT , }
};
}
