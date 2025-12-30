// Generated macro for owned_fd_impl (module)
macro_rules! Depcrate_argowned_fd_impl {
() => {
// Module: crate::arg
// Provides: {"owned_fd_impl"}
// Dependencies: {}
# [cfg (all (unix , not (feature = "stdfd")))] mod owned_fd_impl { use super :: OwnedFd ; use std :: os :: unix :: io :: { RawFd , AsRawFd , FromRawFd , IntoRawFd } ; impl OwnedFd { # [doc = " Create a new OwnedFd from a RawFd."] # [doc = ""] # [doc = " This function is unsafe, because you could potentially send in an invalid file descriptor,"] # [doc = " or close it during the lifetime of this struct. This could potentially be unsound."] pub unsafe fn new (fd : RawFd) -> OwnedFd { OwnedFd { fd : fd } } # [doc = " Convert an OwnedFD back into a RawFd."] pub fn into_fd (self) -> RawFd { let s = self . fd ; :: std :: mem :: forget (self) ; s } # [doc = " Tries to clone the fd."] pub fn try_clone (& self) -> Result < Self , & 'static str > { let x = unsafe { libc :: dup (self . fd) } ; if x == - 1 { Err ("Duplicating file descriptor failed") } else { Ok (unsafe { OwnedFd :: new (x) }) } } } impl Drop for OwnedFd { fn drop (& mut self) { unsafe { libc :: close (self . fd) ; } } } impl AsRawFd for OwnedFd { fn as_raw_fd (& self) -> RawFd { self . fd } } impl IntoRawFd for OwnedFd { fn into_raw_fd (self) -> RawFd { self . into_fd () } } impl FromRawFd for OwnedFd { unsafe fn from_raw_fd (fd : RawFd) -> Self { OwnedFd :: new (fd) } } }
};
}
