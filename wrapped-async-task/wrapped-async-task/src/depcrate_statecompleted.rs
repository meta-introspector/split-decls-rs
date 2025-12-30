// Generated macro for COMPLETED (const)
macro_rules! Depcrate_stateCOMPLETED {
() => {
// Module: crate::state
// Provides: {"COMPLETED"}
// Dependencies: {}
# [doc = " Set if the task has been completed."] # [doc = ""] # [doc = " This flag is set when polling returns `Poll::Ready`. The output of the future is then stored"] # [doc = " inside the task until it becomes closed. In fact, `Task` picks up the output by marking"] # [doc = " the task as closed."] # [doc = ""] # [doc = " This flag can't be set when the task is scheduled or running."] pub (crate) const COMPLETED : usize = 1 << 2 ;
};
}
