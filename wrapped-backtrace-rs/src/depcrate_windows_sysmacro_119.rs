// Generated macro for macro_119 (macro)
macro_rules! Depcrate_windows_sysmacro_119 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_119"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn WaitForSingleObjectEx (hhandle : HANDLE , dwmilliseconds : u32 , balertable : BOOL) -> WAIT_EVENT) ;
};
}
