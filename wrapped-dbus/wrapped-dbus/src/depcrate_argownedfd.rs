// Generated macro for OwnedFd (struct)
macro_rules! Depcrate_argOwnedFd {
() => {
// Module: crate::arg
// Provides: {"OwnedFd"}
// Dependencies: {}
# [doc = " An RAII wrapper around Fd to ensure that file descriptor is closed"] # [doc = " when the scope ends. Enable the `stdfd` feature to use std's OwnedFd instead."] # [cfg (not (feature = "stdfd"))] # [derive (Debug , PartialEq , PartialOrd)] pub struct OwnedFd { # [cfg (unix)] fd : std :: os :: unix :: io :: RawFd }
};
}
