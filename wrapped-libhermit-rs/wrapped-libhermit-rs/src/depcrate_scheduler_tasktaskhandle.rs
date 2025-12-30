// Generated macro for TaskHandle (struct)
macro_rules! Depcrate_scheduler_taskTaskHandle {
() => {
// Module: crate::scheduler::task
// Provides: {"TaskHandle"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub (crate) struct TaskHandle { id : TaskId , priority : Priority , # [cfg (feature = "smp")] core_id : CoreId , }
};
}
