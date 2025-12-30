// Generated macro for Task (struct)
macro_rules! Depcrate_corpusTask {
() => {
// Module: crate::corpus
// Provides: {"Task"}
// Dependencies: {}
# [doc = " Contains all information necessary to run a task."] pub (crate) struct Task { # [doc = " The unique name of the task, which must not be changed after creating it."] # [doc = ""] # [doc = " However, if it is changed it will be treated as new kind of task entirely and won't compare"] # [doc = " to previous runs of the task."] short_name : & 'static str , # [doc = " Explain in greater detail what the task is doing."] description : & 'static str , # [doc = " `true` if the task cannot be run in parallel as it needs all resources by itself."] execute_exclusive : bool , # [doc = " The actual implementation"] execute : & 'static (dyn run :: Execute + Send + Sync) , }
};
}
