// Generated macro for CLOSED (const)
macro_rules! Depcrate_stateCLOSED {
() => {
// Module: crate::state
// Provides: {"CLOSED"}
// Dependencies: {}
# [doc = " Set if the task is closed."] # [doc = ""] # [doc = " If a task is closed, that means it's either canceled or its output has been consumed by the"] # [doc = " `Task`. A task becomes closed in the following cases:"] # [doc = ""] # [doc = " 1. It gets canceled by `Runnable::drop()`, `Task::drop()`, or `Task::cancel()`."] # [doc = " 2. Its output gets awaited by the `Task`."] # [doc = " 3. It panics while polling the future."] # [doc = " 4. It is completed and the `Task` gets dropped."] pub (crate) const CLOSED : usize = 1 << 3 ;
};
}
