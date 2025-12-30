// Generated macro for macro_13 (macro)
macro_rules! Depcrate_bindingsmacro_13 {
() => {
// Module: crate::bindings
// Provides: {"macro_13"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegCreateKeyExW (hkey : HKEY , lpsubkey : PCWSTR , reserved : u32 , lpclass : PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : * const SECURITY_ATTRIBUTES , phkresult : * mut HKEY , lpdwdisposition : * mut REG_CREATE_KEY_DISPOSITION) -> WIN32_ERROR) ;
};
}
