// Generated macro for impl_38 (impl)
macro_rules! Depcrate_fd_guardimpl_38 {
() => {
// Module: crate::fd_guard
// Provides: {"impl_38"}
// Dependencies: {}
impl AsFd for FdGuard { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (self . fd) } } }
};
}
