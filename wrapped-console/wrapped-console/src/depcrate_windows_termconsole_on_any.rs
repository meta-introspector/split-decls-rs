// Generated macro for console_on_any (function)
macro_rules! Depcrate_windows_termconsole_on_any {
() => {
// Module: crate::windows_term
// Provides: {"console_on_any"}
// Dependencies: {}
unsafe fn console_on_any (fds : & [STD_HANDLE]) -> bool { for & fd in fds { let mut out = 0 ; let handle = GetStdHandle (fd) ; if GetConsoleMode (handle , & mut out) != 0 { return true ; } } false }
};
}
