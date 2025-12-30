// Generated macro for macro_23 (macro)
macro_rules! Depcrate_bindingsmacro_23 {
() => {
// Module: crate::bindings
// Provides: {"macro_23"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegRenameKey (hkey : HKEY , lpsubkeyname : PCWSTR , lpnewkeyname : PCWSTR) -> WIN32_ERROR) ;
};
}
