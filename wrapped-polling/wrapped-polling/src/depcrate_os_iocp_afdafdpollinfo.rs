// Generated macro for AfdPollInfo (struct)
macro_rules! Depcrate_os_iocp_afdAfdPollInfo {
() => {
// Module: crate::os::iocp::afd
// Provides: {"AfdPollInfo"}
// Dependencies: {}
# [derive (Default)] # [repr (C)] pub (super) struct AfdPollInfo { # [doc = " The timeout for this poll."] timeout : i64 , # [doc = " The number of handles being polled."] handle_count : u32 , # [doc = " Whether or not this poll is exclusive for this handle."] exclusive : u32 , # [doc = " The handles to poll."] handles : [AfdPollHandleInfo ; 1] , }
};
}
