// Generated macro for macro_35 (macro)
macro_rules! Depcrate_windows_sysmacro_35 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_35"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : PCSTR) -> FARPROC) ;
};
}
