// Generated macro for get_term_size (function)
macro_rules! Depcrateget_term_size {
() => {
// Module: crate
// Provides: {"get_term_size"}
// Dependencies: {}
fn get_term_size (fd : i32) -> Result < (u16 , u16) > { nix :: ioctl_read_bad ! (_get_window_size , libc :: TIOCGWINSZ , winsize) ; let mut size = winsize { ws_col : 0 , ws_row : 0 , ws_xpixel : 0 , ws_ypixel : 0 , } ; let _ = unsafe { _get_window_size (fd , & mut size) } ? ; Ok ((size . ws_col , size . ws_row)) }
};
}
