// Generated macro for IPersist_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_deriveIPersist_Vtbl {
() => {
// Module: crate::interface_cpp_derive
// Provides: {"IPersist_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IPersist_Vtbl { pub base__ : windows_core :: IUnknown_Vtbl , pub GetClassID : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: GUID ,) -> windows_core :: HRESULT , }
};
}
