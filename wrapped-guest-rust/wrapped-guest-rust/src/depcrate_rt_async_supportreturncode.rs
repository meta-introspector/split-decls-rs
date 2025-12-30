// Generated macro for ReturnCode (enum)
macro_rules! Depcrate_rt_async_supportReturnCode {
() => {
// Module: crate::rt::async_support
// Provides: {"ReturnCode"}
// Dependencies: {}
# [doc = " Return code of stream/future operations."] # [derive (PartialEq , Debug , Copy , Clone)] enum ReturnCode { # [doc = " The operation is blocked and has not completed."] Blocked , # [doc = " The operation completed with the specified number of items."] Completed (u32) , # [doc = " The other end is dropped, but before that the specified number of items"] # [doc = " were transferred."] Dropped (u32) , # [doc = " The operation was cancelled, but before that the specified number of"] # [doc = " items were transferred."] Cancelled (u32) , }
};
}
