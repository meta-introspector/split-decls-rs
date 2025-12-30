// Generated macro for TaskData (struct)
macro_rules! Depcrate_taskTaskData {
() => {
// Module: crate::task
// Provides: {"TaskData"}
// Dependencies: {}
# [doc = " A reference to a piece of data that's stored inside of a `Task`."] # [doc = ""] # [doc = " This can be used with the `Task::get` and `Task::get_mut` methods to access"] # [doc = " data inside of tasks."] pub struct TaskData < A > { task_inner : usize , ptr : Arc < UnsafeCell < A > > , }
};
}
