// Generated macro for macro_102 (macro)
macro_rules! Depcrate_windows_sysmacro_102 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_102"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn CreateMutexA (lpmutexattributes : * const SECURITY_ATTRIBUTES , binitialowner : BOOL , lpname : PCSTR) -> HANDLE) ;
};
}
