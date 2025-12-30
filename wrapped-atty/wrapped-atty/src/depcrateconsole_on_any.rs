// Generated macro for console_on_any (function)
macro_rules! Depcrateconsole_on_any {
() => {
// Module: crate
// Provides: {"console_on_any"}
// Dependencies: {}
# [doc = " Returns true if any of the given fds are on a console."] # [cfg (windows)] unsafe fn console_on_any (fds : & [DWORD]) -> bool { use winapi :: um :: { consoleapi :: GetConsoleMode , processenv :: GetStdHandle } ; for & fd in fds { let mut out = 0 ; let handle = GetStdHandle (fd) ; if GetConsoleMode (handle , & mut out) != 0 { return true ; } } false }
};
}
