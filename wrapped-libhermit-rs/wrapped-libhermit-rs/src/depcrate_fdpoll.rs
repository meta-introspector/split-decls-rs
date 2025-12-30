// Generated macro for poll (function)
macro_rules! Depcrate_fdpoll {
() => {
// Module: crate::fd
// Provides: {"poll"}
// Dependencies: {}
# [doc = " Wait for some event on a file descriptor."] # [doc = ""] # [doc = " The unix-like `poll` waits for one of a set of file descriptors"] # [doc = " to become ready to perform I/O. The set of file descriptors to be"] # [doc = " monitored is specified in the `fds` argument, which is an array"] # [doc = " of structs of `PollFd`."] pub fn poll (fds : & mut [PollFd] , timeout : Option < Duration >) -> io :: Result < u64 > { let result = block_on (poll_fds (fds) , timeout) ; if let Err (ref e) = result && timeout . is_some () { if * e == Errno :: Again { return Ok (0) ; } } result }
};
}
