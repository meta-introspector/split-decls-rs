// Generated macro for with_get_last_error (function)
macro_rules! Depcrate_os_windowswith_get_last_error {
() => {
// Module: crate::os::windows
// Provides: {"with_get_last_error"}
// Dependencies: {}
fn with_get_last_error < T , F > (wrap : fn (crate :: error :: WindowsError) -> crate :: Error , closure : F ,) -> Result < T , Option < crate :: Error > > where F : FnOnce () -> Option < T > , { closure () . ok_or_else (| | { let error = unsafe { GetLastError () } ; if error == 0 { None } else { Some (wrap (crate :: error :: WindowsError (error as i32))) } }) }
};
}
