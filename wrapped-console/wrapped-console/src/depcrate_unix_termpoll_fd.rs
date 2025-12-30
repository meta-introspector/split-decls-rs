// Generated macro for poll_fd (function)
macro_rules! Depcrate_unix_termpoll_fd {
() => {
// Module: crate::unix_term
// Provides: {"poll_fd"}
// Dependencies: {}
fn poll_fd (fd : RawFd , timeout : i32) -> io :: Result < bool > { let mut pollfd = libc :: pollfd { fd , events : libc :: POLLIN , revents : 0 , } ; let ret = unsafe { libc :: poll (& mut pollfd as * mut _ , 1 , timeout) } ; if ret < 0 { Err (io :: Error :: last_os_error ()) } else { Ok (pollfd . revents & libc :: POLLIN != 0) } }
};
}
