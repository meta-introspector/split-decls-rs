// Generated macro for AfdPollHandleInfo (struct)
macro_rules! Depcrate_os_iocp_afdAfdPollHandleInfo {
() => {
// Module: crate::os::iocp::afd
// Provides: {"AfdPollHandleInfo"}
// Dependencies: {}
# [repr (C)] struct AfdPollHandleInfo { # [doc = " The handle to poll."] handle : HANDLE , # [doc = " The events to poll for."] events : AfdPollMask , # [doc = " The status of the poll."] status : NTSTATUS , }
};
}
