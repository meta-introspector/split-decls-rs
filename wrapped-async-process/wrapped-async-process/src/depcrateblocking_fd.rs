// Generated macro for blocking_fd (function)
macro_rules! Depcrateblocking_fd {
() => {
// Module: crate
// Provides: {"blocking_fd"}
// Dependencies: {}
# [doc = " Moves `Fd` out of non-blocking mode."] # [cfg (unix)] fn blocking_fd (fd : rustix :: fd :: BorrowedFd < '_ >) -> io :: Result < () > { cfg_if :: cfg_if ! { if # [cfg (target_os = "linux")] { rustix :: io :: ioctl_fionbio (fd , false) ?; } else { let previous = rustix :: fs :: fcntl_getfl (fd) ?; let new = previous & ! rustix :: fs :: OFlags :: NONBLOCK ; if new != previous { rustix :: fs :: fcntl_setfl (fd , new) ?; } } } Ok (()) }
};
}
