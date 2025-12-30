// Generated macro for SCHEDULED (const)
macro_rules! Depcrate_stateSCHEDULED {
() => {
// Module: crate::state
// Provides: {"SCHEDULED"}
// Dependencies: {}
# [doc = " Set if the task is scheduled for running."] # [doc = ""] # [doc = " A task is considered to be scheduled whenever its `Runnable` exists."] # [doc = ""] # [doc = " This flag can't be set when the task is completed. However, it can be set while the task is"] # [doc = " running, in which case it will be rescheduled as soon as polling finishes."] pub (crate) const SCHEDULED : usize = 1 << 0 ;
};
}
