// Generated macro for IPersist_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_sys_no_coreIPersist_Vtbl {
() => {
// Module: crate::interface_cpp_sys_no_core
// Provides: {"IPersist_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IPersist_Vtbl { pub base__ : IUnknown_Vtbl , pub GetClassID : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut GUID) -> HRESULT , }
};
}
