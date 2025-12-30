// Generated macro for sys_exit (function)
macro_rules! Depcrate_syscalls_taskssys_exit {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_exit"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_exit (status : i32) -> ! { exit (status) }
};
}
