// Generated macro for show_cursor (function)
macro_rules! Depcrate_windows_termshow_cursor {
() => {
// Module: crate::windows_term
// Provides: {"show_cursor"}
// Dependencies: {}
pub (crate) fn show_cursor (out : & Term) -> io :: Result < () > { if out . is_msys_tty { return common_term :: show_cursor (out) ; } if let Some ((hand , mut cci)) = get_console_cursor_info (as_handle (out)) { unsafe { cci . bVisible = 1 ; SetConsoleCursorInfo (hand , & cci) ; } } Ok (()) }
};
}
