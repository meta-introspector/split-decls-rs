// Generated macro for sys_get_priority (function)
macro_rules! Depcrate_syscalls_taskssys_get_priority {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_get_priority"}
// Dependencies: {}
# [doc = " Determine the priority of the current thread"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_get_priority () -> u8 { core_scheduler () . get_current_task_prio () . into () }
};
}
