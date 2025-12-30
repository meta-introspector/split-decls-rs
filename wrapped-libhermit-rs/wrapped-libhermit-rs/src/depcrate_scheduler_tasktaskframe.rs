// Generated macro for TaskFrame (trait)
macro_rules! Depcrate_scheduler_taskTaskFrame {
() => {
// Module: crate::scheduler::task
// Provides: {"TaskFrame"}
// Dependencies: {}
pub (crate) trait TaskFrame { # [doc = " Create the initial stack frame for a new task"] fn create_stack_frame (& mut self , func : unsafe extern "C" fn (usize) , arg : usize) ; }
};
}
