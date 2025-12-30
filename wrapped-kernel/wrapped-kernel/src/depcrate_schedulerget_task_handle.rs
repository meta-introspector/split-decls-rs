// Generated macro for get_task_handle (function)
macro_rules! Depcrate_schedulerget_task_handle {
() => {
// Module: crate::scheduler
// Provides: {"get_task_handle"}
// Dependencies: {}
fn get_task_handle (id : TaskId) -> Option < TaskHandle > { TASKS . lock () . get (& id) . copied () }
};
}
