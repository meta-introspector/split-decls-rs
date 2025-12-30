// Generated macro for macro_99 (macro)
macro_rules! Depcrate_windows_sysmacro_99 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_99"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymSetSearchPathW (hprocess : HANDLE , searchpatha : PCWSTR) -> BOOL) ;
};
}
