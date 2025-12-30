// Generated macro for terminal_size (function)
macro_rules! Depcrate_unix_termterminal_size {
() => {
// Module: crate::unix_term
// Provides: {"terminal_size"}
// Dependencies: {}
pub (crate) fn terminal_size (out : & Term) -> Option < (u16 , u16) > { if ! is_a_terminal (out) { return None ; } let winsize = unsafe { let mut winsize : libc :: winsize = mem :: zeroed () ; # [allow (clippy :: useless_conversion)] libc :: ioctl (out . as_raw_fd () , libc :: TIOCGWINSZ . into () , & mut winsize) ; winsize } ; if winsize . ws_row > 0 && winsize . ws_col > 0 { Some ((winsize . ws_row , winsize . ws_col)) } else { None } }
};
}
