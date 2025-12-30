// Generated macro for sys_set_priority (function)
macro_rules! Depcrate_syscalls_taskssys_set_priority {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_set_priority"}
// Dependencies: {}
# [doc = " Set priority of the thread with the identifier `id`"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_set_priority (id : Tid , prio : u8) { if prio > 0 { core_scheduler () . set_priority (TaskId :: from (id) , Priority :: from (prio)) . expect ("Unable to set priority") ; } else { panic ! ("Invalid priority {}" , prio) ; } }
};
}
