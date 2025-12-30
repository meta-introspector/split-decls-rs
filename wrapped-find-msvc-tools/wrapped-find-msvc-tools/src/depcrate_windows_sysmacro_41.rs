// Generated macro for macro_41 (macro)
macro_rules! Depcrate_windows_sysmacro_41 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_41"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyExW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY) -> WIN32_ERROR) ;
};
}
