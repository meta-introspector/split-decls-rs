// Generated macro for sys_set_current_task_priority (function)
macro_rules! Depcrate_syscalls_taskssys_set_current_task_priority {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_set_current_task_priority"}
// Dependencies: {}
# [doc = " Set priority of the current thread"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_set_current_task_priority (prio : u8) { if prio > 0 { core_scheduler () . set_current_task_priority (Priority :: from (prio)) ; } else { panic ! ("Invalid priority {}" , prio) ; } }
};
}
