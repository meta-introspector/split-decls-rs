// Generated macro for macro_31 (macro)
macro_rules! Depcrate_windows_sysmacro_31 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_31"}
// Dependencies: {}
windows_link :: link ! ("ole32.dll" "system" fn CoCreateInstance (rclsid : * const GUID , punkouter : * mut core :: ffi :: c_void , dwclscontext : CLSCTX , riid : * const GUID , ppv : * mut * mut core :: ffi :: c_void) -> HRESULT) ;
};
}
