// Generated macro for sys_yield (function)
macro_rules! Depcrate_syscalls_taskssys_yield {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_yield"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_yield () { core_scheduler () . reschedule () ; }
};
}
