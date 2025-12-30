// Generated macro for macro_42 (macro)
macro_rules! Depcrate_windows_sysmacro_42 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_42"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegQueryValueExW (hkey : HKEY , lpvaluename : PCWSTR , lpreserved : * const u32 , lptype : * mut REG_VALUE_TYPE , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
};
}
