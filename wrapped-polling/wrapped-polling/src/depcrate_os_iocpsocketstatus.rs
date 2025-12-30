// Generated macro for SocketStatus (enum)
macro_rules! Depcrate_os_iocpSocketStatus {
() => {
// Module: crate::os::iocp
// Provides: {"SocketStatus"}
// Dependencies: {}
# [doc = " The mode that a socket can be in."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] enum SocketStatus { # [doc = " We are currently not polling."] Idle , # [doc = " We are currently polling these events."] Polling { # [doc = " The flags we are currently polling for."] flags : AfdPollMask , } , # [doc = " The last poll operation was cancelled, and we're waiting for it to"] # [doc = " complete."] Cancelled , }
};
}
