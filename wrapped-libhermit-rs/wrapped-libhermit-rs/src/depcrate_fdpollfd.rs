// Generated macro for PollFd (struct)
macro_rules! Depcrate_fdPollFd {
() => {
// Module: crate::fd
// Provides: {"PollFd"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct PollFd { # [doc = " file descriptor"] pub fd : i32 , # [doc = " events to look for"] pub events : PollEvent , # [doc = " events returned"] pub revents : PollEvent , }
};
}
