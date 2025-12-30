// Generated macro for read_single_char (function)
macro_rules! Depcrate_unix_termread_single_char {
() => {
// Module: crate::unix_term
// Provides: {"read_single_char"}
// Dependencies: {}
fn read_single_char (fd : RawFd) -> io :: Result < Option < char > > { let is_ready = select_or_poll_term_fd (fd , 0) ? ; if is_ready { let mut buf : [u8 ; 1] = [0] ; read_bytes (fd , & mut buf , 1) ? ; Ok (Some (buf [0] as char)) } else { Ok (None) } }
};
}
