// Generated macro for macro_37 (macro)
macro_rules! Depcrate_windows_sysmacro_37 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_37"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn OpenSemaphoreA (dwdesiredaccess : u32 , binherithandle : BOOL , lpname : PCSTR) -> HANDLE) ;
};
}
