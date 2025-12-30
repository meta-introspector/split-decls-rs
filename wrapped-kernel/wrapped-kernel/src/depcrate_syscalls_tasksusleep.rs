// Generated macro for usleep (function)
macro_rules! Depcrate_syscalls_tasksusleep {
() => {
// Module: crate::syscalls::tasks
// Provides: {"usleep"}
// Dependencies: {}
pub (super) fn usleep (usecs : u64) { if usecs >= 10_000 { debug ! ("sys_usleep blocking the task for {usecs} microseconds") ; let wakeup_time = arch :: processor :: get_timer_ticks () + usecs ; let core_scheduler = core_scheduler () ; core_scheduler . block_current_task (Some (wakeup_time)) ; core_scheduler . reschedule () ; } else if usecs > 0 { let end = arch :: processor :: get_timestamp () + u64 :: from (get_frequency ()) * usecs ; while get_timestamp () < end { core_scheduler () . reschedule () ; } } }
};
}
