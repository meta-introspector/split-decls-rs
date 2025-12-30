// Generated macro for macro_43 (macro)
macro_rules! Depcrate_windows_sysmacro_43 {
() => {
// Module: crate::windows_sys
// Provides: {"macro_43"}
// Dependencies: {}
windows_link :: link ! ("kernel32.dll" "system" fn ReleaseSemaphore (hsemaphore : HANDLE , lreleasecount : i32 , lppreviouscount : * mut i32) -> BOOL) ;
};
}
