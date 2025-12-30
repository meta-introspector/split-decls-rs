// Generated macro for sys_wakeup_task (function)
macro_rules! Depcrate_syscalls_taskssys_wakeup_task {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_wakeup_task"}
// Dependencies: {}
# [doc = " Wake up the task with the identifier `id`"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_wakeup_task (id : Tid) { let task_id = TaskId :: from (id) ; if let Some (handle) = BLOCKED_TASKS . lock () . remove (& task_id) { core_scheduler () . custom_wakeup (handle) ; } }
};
}
