// Generated macro for SetConsoleCtrlHandler (function)
macro_rules! Depcrate_delegate_paramSetConsoleCtrlHandler {
() => {
// Module: crate::delegate_param
// Provides: {"SetConsoleCtrlHandler"}
// Dependencies: {}
# [inline] pub unsafe fn SetConsoleCtrlHandler (handlerroutine : PHANDLER_ROUTINE , add : bool ,) -> windows_core :: Result < () > { windows_core :: link ! ("kernel32.dll" "system" fn SetConsoleCtrlHandler (handlerroutine : PHANDLER_ROUTINE , add : windows_core :: BOOL) -> windows_core :: BOOL) ; unsafe { SetConsoleCtrlHandler (handlerroutine , add . into ()) . ok () } }
};
}
