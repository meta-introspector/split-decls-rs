// Generated macro for select_or_poll_term_fd (function)
macro_rules! Depcrate_unix_termselect_or_poll_term_fd {
() => {
// Module: crate::unix_term
// Provides: {"select_or_poll_term_fd"}
// Dependencies: {}
fn select_or_poll_term_fd (fd : RawFd , timeout : i32) -> io :: Result < bool > { # [cfg (target_os = "macos")] { if unsafe { libc :: isatty (fd) == 1 } { return select_fd (fd , timeout) ; } } poll_fd (fd , timeout) }
};
}
