// Generated macro for select_fd (function)
macro_rules! Depcrate_unix_termselect_fd {
() => {
// Module: crate::unix_term
// Provides: {"select_fd"}
// Dependencies: {}
# [cfg (target_os = "macos")] fn select_fd (fd : RawFd , timeout : i32) -> io :: Result < bool > { unsafe { let mut read_fd_set : libc :: fd_set = mem :: zeroed () ; let mut timeout_val ; let timeout = if timeout < 0 { ptr :: null_mut () } else { timeout_val = libc :: timeval { tv_sec : (timeout / 1000) as _ , tv_usec : (timeout * 1000) as _ , } ; & mut timeout_val } ; libc :: FD_ZERO (& mut read_fd_set) ; libc :: FD_SET (fd , & mut read_fd_set) ; let ret = libc :: select (fd + 1 , & mut read_fd_set , ptr :: null_mut () , ptr :: null_mut () , timeout ,) ; if ret < 0 { Err (io :: Error :: last_os_error ()) } else { Ok (libc :: FD_ISSET (fd , & read_fd_set)) } } }
};
}
