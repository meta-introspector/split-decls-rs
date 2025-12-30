// Generated macro for TaskLayout (struct)
macro_rules! Depcrate_rawTaskLayout {
() => {
// Module: crate::raw
// Provides: {"TaskLayout"}
// Dependencies: {}
# [doc = " Memory layout of a task."] # [doc = ""] # [doc = " This struct contains the following information:"] # [doc = ""] # [doc = " 1. How to allocate and deallocate the task."] # [doc = " 2. How to access the fields inside the task."] # [derive (Clone , Copy)] pub (crate) struct TaskLayout { # [doc = " Memory layout of the whole task."] pub (crate) layout : StdLayout , # [doc = " Offset into the task at which the schedule function is stored."] pub (crate) offset_s : usize , # [doc = " Offset into the task at which the future is stored."] pub (crate) offset_f : usize , # [doc = " Offset into the task at which the output is stored."] pub (crate) offset_r : usize , }
};
}
