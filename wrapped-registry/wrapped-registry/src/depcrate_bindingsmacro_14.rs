// Generated macro for macro_14 (macro)
macro_rules! Depcrate_bindingsmacro_14 {
() => {
// Module: crate::bindings
// Provides: {"macro_14"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegCreateKeyTransactedW (hkey : HKEY , lpsubkey : PCWSTR , reserved : u32 , lpclass : PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : * const SECURITY_ATTRIBUTES , phkresult : * mut HKEY , lpdwdisposition : * mut REG_CREATE_KEY_DISPOSITION , htransaction : HANDLE , pextendedparemeter : * const core :: ffi :: c_void) -> WIN32_ERROR) ;
};
}
