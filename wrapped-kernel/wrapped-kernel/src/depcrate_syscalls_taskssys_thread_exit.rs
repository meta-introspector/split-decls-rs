// Generated macro for sys_thread_exit (function)
macro_rules! Depcrate_syscalls_taskssys_thread_exit {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_thread_exit"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_thread_exit (status : i32) -> ! { debug ! ("Exit thread with error code {status}!") ; core_scheduler () . exit (status) }
};
}
