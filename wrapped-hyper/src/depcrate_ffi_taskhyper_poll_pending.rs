// Generated macro for HYPER_POLL_PENDING (const)
macro_rules! Depcrate_ffi_taskHYPER_POLL_PENDING {
() => {
// Module: crate::ffi::task
// Provides: {"HYPER_POLL_PENDING"}
// Dependencies: {}
# [doc = " Return in a poll function to indicate it is still pending."] # [doc = ""] # [doc = " The passed in `hyper_waker` should be registered to wake up the task at"] # [doc = " some later point."] pub const HYPER_POLL_PENDING : c_int = 1 ;
};
}
