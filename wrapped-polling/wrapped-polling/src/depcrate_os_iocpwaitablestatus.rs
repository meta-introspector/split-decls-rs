// Generated macro for WaitableStatus (enum)
macro_rules! Depcrate_os_iocpWaitableStatus {
() => {
// Module: crate::os::iocp
// Provides: {"WaitableStatus"}
// Dependencies: {}
# [derive (Debug)] enum WaitableStatus { # [doc = " We are not polling."] Idle , # [doc = " We are waiting on this handle to become signaled."] Waiting (# [allow (dead_code)] WaitHandle) , # [doc = " This handle has been cancelled."] Cancelled , }
};
}
