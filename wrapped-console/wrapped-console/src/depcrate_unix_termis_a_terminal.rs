// Generated macro for is_a_terminal (function)
macro_rules! Depcrate_unix_termis_a_terminal {
() => {
// Module: crate::unix_term
// Provides: {"is_a_terminal"}
// Dependencies: {}
# [inline] pub (crate) fn is_a_terminal (out : & impl AsRawFd) -> bool { unsafe { libc :: isatty (out . as_raw_fd ()) != 0 } }
};
}
