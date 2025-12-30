// Generated macro for macro_24 (macro)
macro_rules! Depcrate_bindingsmacro_24 {
() => {
// Module: crate::bindings
// Provides: {"macro_24"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegSetValueExW (hkey : HKEY , lpvaluename : PCWSTR , reserved : u32 , dwtype : REG_VALUE_TYPE , lpdata : * const u8 , cbdata : u32) -> WIN32_ERROR) ;
};
}
