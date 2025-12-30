// Generated macro for TaskQueue (struct)
macro_rules! Depcrate_task_poolTaskQueue {
() => {
// Module: crate::task_pool
// Provides: {"TaskQueue"}
// Dependencies: {}
# [doc = " `TaskQueue`, like its name suggests, queues tasks."] # [doc = ""] # [doc = " This should only be used if a task must run after [`GlobalState::process_changes`]"] # [doc = " has been called."] pub (crate) struct TaskQueue { pub (crate) sender : crossbeam_channel :: Sender < QueuedTask > , pub (crate) receiver : crossbeam_channel :: Receiver < QueuedTask > , }
};
}
