// Generated macro for block_current_task (function)
macro_rules! Depcrate_syscalls_tasksblock_current_task {
() => {
// Module: crate::syscalls::tasks
// Provides: {"block_current_task"}
// Dependencies: {}
fn block_current_task (timeout : Option < u64 >) { let wakeup_time = timeout . map (| t | arch :: processor :: get_timer_ticks () + t * 1000) ; let core_scheduler = core_scheduler () ; let handle = core_scheduler . get_current_task_handle () ; let tid = core_scheduler . get_current_task_id () ; BLOCKED_TASKS . lock () . insert (tid , handle) ; core_scheduler . block_current_task (wakeup_time) ; }
};
}
