// Generated macro for macro_22 (macro)
macro_rules! Depcrate_bindingsmacro_22 {
() => {
// Module: crate::bindings
// Provides: {"macro_22"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegQueryValueExW (hkey : HKEY , lpvaluename : PCWSTR , lpreserved : * const u32 , lptype : * mut REG_VALUE_TYPE , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
};
}
