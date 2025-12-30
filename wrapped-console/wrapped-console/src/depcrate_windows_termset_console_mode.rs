// Generated macro for set_console_mode (function)
macro_rules! Depcrate_windows_termset_console_mode {
() => {
// Module: crate::windows_term
// Provides: {"set_console_mode"}
// Dependencies: {}
# [doc = " Enables or disables the `mode` flag on the given `HANDLE` and yields the previous mode."] fn set_console_mode (handle : HANDLE , mode : CONSOLE_MODE , enable : bool) -> Option < CONSOLE_MODE > { unsafe { let mut dw_mode = 0 ; if GetConsoleMode (handle , & mut dw_mode) == 0 { return None ; } let new_dw_mode = match enable { true => dw_mode | mode , false => dw_mode & ! mode , } ; if SetConsoleMode (handle , new_dw_mode) == 0 { return None ; } Some (dw_mode) } }
};
}
