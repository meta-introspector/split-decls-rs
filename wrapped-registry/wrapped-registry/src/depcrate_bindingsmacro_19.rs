// Generated macro for macro_19 (macro)
macro_rules! Depcrate_bindingsmacro_19 {
() => {
// Module: crate::bindings
// Provides: {"macro_19"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyExW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY) -> WIN32_ERROR) ;
};
}
