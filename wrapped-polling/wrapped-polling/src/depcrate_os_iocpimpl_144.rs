// Generated macro for impl_144 (impl)
macro_rules! Depcrate_os_iocpimpl_144 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_144"}
// Dependencies: {}
impl AsHandle for Poller { fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
