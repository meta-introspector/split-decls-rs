// Generated macro for macro_46 (macro)
macro_rules! Depcrate_windows_sysmacro_46 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_46"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn WaitForSingleObject (hhandle : HANDLE , dwmilliseconds : u32) -> WAIT_EVENT) ;
};
}
