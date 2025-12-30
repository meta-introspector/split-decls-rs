// Generated macro for GetLastError (function)
macro_rules! Depcrate_bindingsGetLastError {
() => {
// Module: crate::bindings
// Provides: {"GetLastError"}
// Dependencies: {}
# [inline] pub unsafe fn GetLastError () -> windows_core :: WIN32_ERROR { windows_core :: link ! ("kernel32.dll" "system" fn GetLastError () -> windows_core :: WIN32_ERROR) ; unsafe { GetLastError () } }
};
}
