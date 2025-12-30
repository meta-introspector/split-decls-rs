// Generated macro for set_nonblocking (function)
macro_rules! Depcrateset_nonblocking {
() => {
// Module: crate
// Provides: {"set_nonblocking"}
// Dependencies: {}
# [inline] fn set_nonblocking (# [cfg (unix)] fd : BorrowedFd < '_ > , # [cfg (windows)] fd : BorrowedSocket < '_ > ,) -> io :: Result < () > { cfg_if :: cfg_if ! { if # [cfg (any (windows , target_os = "linux"))] { rustix :: io :: ioctl_fionbio (fd , true) ?; } else { let previous = rustix :: fs :: fcntl_getfl (fd) ?; let new = previous | rustix :: fs :: OFlags :: NONBLOCK ; if new != previous { rustix :: fs :: fcntl_setfl (fd , new) ?; } } } Ok (()) }
};
}
