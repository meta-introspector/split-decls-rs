// Generated macro for get_eventfd (function)
macro_rules! Depcrate_tests_register_sync_cancelget_eventfd {
() => {
// Module: crate::tests::register_sync_cancel
// Provides: {"get_eventfd"}
// Dependencies: {}
# [doc = " Setup an eventfd which we can use for a blocking read."] fn get_eventfd () -> OwnedFd { let fd = unsafe { libc :: eventfd (0 , 0) } ; assert ! (fd >= 0) ; unsafe { OwnedFd :: from_raw_fd (fd) } }
};
}
