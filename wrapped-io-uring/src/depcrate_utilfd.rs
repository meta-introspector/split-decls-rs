// Generated macro for fd (module)
macro_rules! Depcrate_utilfd {
() => {
// Module: crate::util
// Provides: {"fd"}
// Dependencies: {}
# [cfg (not (feature = "io_safety"))] mod fd { use std :: mem ; use std :: os :: unix :: io :: { AsRawFd , FromRawFd , IntoRawFd , RawFd } ; # [doc = " API-compatible with the `OwnedFd` type in the Rust stdlib."] pub struct OwnedFd (RawFd) ; impl AsRawFd for OwnedFd { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 } } impl IntoRawFd for OwnedFd { # [inline] fn into_raw_fd (self) -> RawFd { let fd = self . 0 ; mem :: forget (self) ; fd } } impl FromRawFd for OwnedFd { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> OwnedFd { OwnedFd (fd) } } impl Drop for OwnedFd { fn drop (& mut self) { unsafe { libc :: close (self . 0) ; } } } }
};
}
