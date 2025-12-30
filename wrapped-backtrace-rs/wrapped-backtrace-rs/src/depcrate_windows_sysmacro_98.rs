// Generated macro for macro_98 (macro)
macro_rules! Depcrate_windows_sysmacro_98 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_98"}
// Dependencies: {}
windows_link :: link ! ("dbghelp.dll" "system" fn SymSetOptions (symoptions : u32) -> u32) ;
};
}
