// Generated macro for IPersistFile_Vtbl (struct)
macro_rules! Depcrate_interface_cpp_deriveIPersistFile_Vtbl {
() => {
// Module: crate::interface_cpp_derive
// Provides: {"IPersistFile_Vtbl"}
// Dependencies: {}
# [repr (C)] # [doc (hidden)] pub struct IPersistFile_Vtbl { pub base__ : IPersist_Vtbl , pub IsDirty : unsafe extern "system" fn (* mut core :: ffi :: c_void) -> windows_core :: HRESULT , Load : usize , pub Save : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR , windows_core :: BOOL ,) -> windows_core :: HRESULT , pub SaveCompleted : unsafe extern "system" fn (* mut core :: ffi :: c_void , windows_core :: PCWSTR ,) -> windows_core :: HRESULT , pub GetCurFile : unsafe extern "system" fn (* mut core :: ffi :: c_void , * mut windows_core :: PWSTR ,) -> windows_core :: HRESULT , }
};
}
