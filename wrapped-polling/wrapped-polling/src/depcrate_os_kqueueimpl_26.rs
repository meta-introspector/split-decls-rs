// Generated macro for impl_26 (impl)
macro_rules! Depcrate_os_kqueueimpl_26 {
() => {
// Module: crate::os::kqueue
// Provides: {"impl_26"}
// Dependencies: {}
impl AsFd for Poller { fn as_fd (& self) -> BorrowedFd < '_ > { self . kqueue_fd . as_fd () } }
};
}
