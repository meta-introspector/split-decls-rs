// Generated macro for macro_20 (macro)
macro_rules! Depcrate_bindingsmacro_20 {
() => {
// Module: crate::bindings
// Provides: {"macro_20"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyTransactedW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY , htransaction : HANDLE , pextendedparemeter : * const core :: ffi :: c_void) -> WIN32_ERROR) ;
};
}
