// Generated macro for macro_107 (macro)
macro_rules! Depcrate_windows_sysmacro_107 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_107"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : PCSTR) -> FARPROC) ;
};
}
