// Generated macro for TaskStatus (enum)
macro_rules! Depcrate_scheduler_taskTaskStatus {
() => {
// Module: crate::scheduler::task
// Provides: {"TaskStatus"}
// Dependencies: {}
# [doc = " The status of the task - used for scheduling"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum TaskStatus { Invalid , Ready , Running , Blocked , Finished , Idle , }
};
}
