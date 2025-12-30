// Generated macro for macro_16 (macro)
macro_rules! Depcrate_bindingsmacro_16 {
() => {
// Module: crate::bindings
// Provides: {"macro_16"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegDeleteValueW (hkey : HKEY , lpvaluename : PCWSTR) -> WIN32_ERROR) ;
};
}
