// Generated macro for sys_getprio (function)
macro_rules! Depcrate_syscalls_taskssys_getprio {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_getprio"}
// Dependencies: {}
# [cfg (feature = "newlib")] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_getprio (id : * const Tid) -> i32 { let task = core_scheduler () . get_current_task_handle () ; if id . is_null () || unsafe { * id } == task . get_id () . into () { i32 :: from (task . get_priority () . into ()) } else { - i32 :: from (Errno :: Inval) } }
};
}
