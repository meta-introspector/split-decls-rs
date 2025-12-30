// Generated macro for GetLastError (function)
macro_rules! Depcrate_bindingsGetLastError {
() => {
// Module: crate::bindings
// Provides: {"GetLastError"}
// Dependencies: {}
# [inline] pub unsafe fn GetLastError () -> windows_result :: WIN32_ERROR { windows_link :: link ! ("kernel32.dll" "system" fn GetLastError () -> windows_result :: WIN32_ERROR) ; unsafe { GetLastError () } }
};
}
