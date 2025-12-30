// Generated macro for macro_18 (macro)
macro_rules! Depcrate_bindingsmacro_18 {
() => {
// Module: crate::bindings
// Provides: {"macro_18"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegEnumValueW (hkey : HKEY , dwindex : u32 , lpvaluename : PWSTR , lpcchvaluename : * mut u32 , lpreserved : * const u32 , lptype : * mut u32 , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
};
}
