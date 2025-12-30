// Generated macro for SchedulerInput (struct)
macro_rules! Depcrate_schedulerSchedulerInput {
() => {
// Module: crate::scheduler
// Provides: {"SchedulerInput"}
// Dependencies: {}
# [cfg (feature = "smp")] pub (crate) struct SchedulerInput { # [doc = " Queue of new tasks"] new_tasks : VecDeque < NewTask > , # [doc = " Queue of task, which are wakeup by another core"] wakeup_tasks : VecDeque < TaskHandle > , }
};
}
