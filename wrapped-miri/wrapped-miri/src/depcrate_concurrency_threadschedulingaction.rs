// Generated macro for SchedulingAction (enum)
macro_rules! Depcrate_concurrency_threadSchedulingAction {
() => {
// Module: crate::concurrency::thread
// Provides: {"SchedulingAction"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] enum SchedulingAction { # [doc = " Execute step on the active thread."] ExecuteStep , # [doc = " Execute a timeout callback."] ExecuteTimeoutCallback , # [doc = " Wait for a bit, until there is a timeout to be called."] Sleep (Duration) , }
};
}
