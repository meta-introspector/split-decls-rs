// Generated macro for sys_usleep (function)
macro_rules! Depcrate_syscalls_taskssys_usleep {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_usleep"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_usleep (usecs : u64) { usleep (usecs) ; }
};
}
