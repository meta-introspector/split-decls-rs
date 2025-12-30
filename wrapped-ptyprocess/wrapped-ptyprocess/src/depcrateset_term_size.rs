// Generated macro for set_term_size (function)
macro_rules! Depcrateset_term_size {
() => {
// Module: crate
// Provides: {"set_term_size"}
// Dependencies: {}
fn set_term_size (fd : i32 , cols : u16 , rows : u16) -> Result < () > { ioctl_write_ptr_bad ! (_set_window_size , libc :: TIOCSWINSZ , winsize) ; let size = winsize { ws_row : rows , ws_col : cols , ws_xpixel : 0 , ws_ypixel : 0 , } ; let _ = unsafe { _set_window_size (fd , & size) } ? ; Ok (()) }
};
}
