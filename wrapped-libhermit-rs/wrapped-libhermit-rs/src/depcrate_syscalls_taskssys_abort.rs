// Generated macro for sys_abort (function)
macro_rules! Depcrate_syscalls_taskssys_abort {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_abort"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_abort () -> ! { exit (- 1) }
};
}
