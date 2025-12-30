// Generated macro for impl_32 (impl)
macro_rules! Depcrate_fd_guardimpl_32 {
() => {
// Module: crate::fd_guard
// Provides: {"impl_32"}
// Dependencies: {}
impl FdGuard { # [doc = " Indicate that the wrapped file descriptor should _not_ be closed"] # [doc = " when the guard is dropped."] # [doc = ""] # [doc = " This should be called in cases where ownership of the wrapped file"] # [doc = " descriptor has been \"moved\" out of the guard."] # [doc = ""] # [doc = " This is factored out into a separate function to ensure that it's"] # [doc = " always used consistently."] # [inline] pub fn should_not_close (& self) { self . close_on_drop . store (false , Ordering :: Release) ; } }
};
}
