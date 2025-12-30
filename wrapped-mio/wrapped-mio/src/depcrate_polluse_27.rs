// Generated macro for use_27 (use)
macro_rules! Depcrate_polluse_27 {
() => {
// Module: crate::poll
// Provides: {"use_27"}
// Dependencies: {}
# [cfg (all (unix , not (mio_unsupported_force_poll_poll) , not (any (target_os = "aix" , target_os = "espidf" , target_os = "fuchsia" , target_os = "haiku" , target_os = "hermit" , target_os = "hurd" , target_os = "nto" , target_os = "solaris" , target_os = "vita" , target_os = "cygwin" ,)) ,))] use std :: os :: fd :: { AsRawFd , RawFd } ;
};
}
