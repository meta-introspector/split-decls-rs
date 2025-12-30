// Generated macro for hide_cursor (function)
macro_rules! Depcrate_windows_termhide_cursor {
() => {
// Module: crate::windows_term
// Provides: {"hide_cursor"}
// Dependencies: {}
pub (crate) fn hide_cursor (out : & Term) -> io :: Result < () > { if out . is_msys_tty { return common_term :: hide_cursor (out) ; } if let Some ((hand , mut cci)) = get_console_cursor_info (as_handle (out)) { unsafe { cci . bVisible = 0 ; SetConsoleCursorInfo (hand , & cci) ; } } Ok (()) }
};
}
