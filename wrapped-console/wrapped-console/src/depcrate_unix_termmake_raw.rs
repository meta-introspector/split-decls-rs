// Generated macro for make_raw (function)
macro_rules! Depcrate_unix_termmake_raw {
() => {
// Module: crate::unix_term
// Provides: {"make_raw"}
// Dependencies: {}
# [cfg (not (target_os = "nto"))] fn make_raw (termios : & mut libc :: termios) { unsafe { libc :: cfmakeraw (termios) } ; }
};
}
