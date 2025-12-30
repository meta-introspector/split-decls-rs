// Generated macro for RUNNING (const)
macro_rules! Depcrate_stateRUNNING {
() => {
// Module: crate::state
// Provides: {"RUNNING"}
// Dependencies: {}
# [doc = " Set if the task is running."] # [doc = ""] # [doc = " A task is in running state while its future is being polled."] # [doc = ""] # [doc = " This flag can't be set when the task is completed. However, it can be in scheduled state while"] # [doc = " it is running, in which case it will be rescheduled as soon as polling finishes."] pub (crate) const RUNNING : usize = 1 << 1 ;
};
}
