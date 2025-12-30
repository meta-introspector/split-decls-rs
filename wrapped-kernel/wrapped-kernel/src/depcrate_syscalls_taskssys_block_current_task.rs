// Generated macro for sys_block_current_task (function)
macro_rules! Depcrate_syscalls_taskssys_block_current_task {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_block_current_task"}
// Dependencies: {}
# [doc = " Set the current task state to `blocked`"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_block_current_task () { block_current_task (None) ; }
};
}
