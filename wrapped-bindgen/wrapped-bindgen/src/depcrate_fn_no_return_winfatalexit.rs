// Generated macro for FatalExit (function)
macro_rules! Depcrate_fn_no_return_winFatalExit {
() => {
// Module: crate::fn_no_return_win
// Provides: {"FatalExit"}
// Dependencies: {}
# [inline] pub unsafe fn FatalExit (exitcode : i32) -> ! { windows_core :: link ! ("kernel32.dll" "system" fn FatalExit (exitcode : i32) -> !) ; unsafe { FatalExit (exitcode) } }
};
}
