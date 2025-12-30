// Generated macro for sys_block_current_task_with_timeout (function)
macro_rules! Depcrate_syscalls_taskssys_block_current_task_with_timeout {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_block_current_task_with_timeout"}
// Dependencies: {}
# [doc = " Set the current task state to `blocked`"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_block_current_task_with_timeout (timeout : u64) { block_current_task (Some (timeout)) ; }
};
}
