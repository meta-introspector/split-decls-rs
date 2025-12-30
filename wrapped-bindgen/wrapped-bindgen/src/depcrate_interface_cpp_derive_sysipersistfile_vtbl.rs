// Generated macro for IPersistFile_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_derive_sysIPersistFile_Vtbl {
() => {
// Module: crate::interface_cpp_derive_sys
// Provides: {"IPersistFile_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IPersistFile_Vtbl { pub base__ : IPersist_Vtbl , pub IsDirty : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_sys :: core :: HRESULT , Load : usize , pub Save : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_sys :: core :: PCWSTR , windows_sys :: core :: BOOL ,) -> windows_sys :: core :: HRESULT , pub SaveCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_sys :: core :: PCWSTR ,) -> windows_sys :: core :: HRESULT , pub GetCurFile : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_sys :: core :: PWSTR ,) -> windows_sys :: core :: HRESULT , }
};
}
