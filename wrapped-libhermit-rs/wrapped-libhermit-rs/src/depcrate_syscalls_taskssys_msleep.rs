// Generated macro for sys_msleep (function)
macro_rules! Depcrate_syscalls_taskssys_msleep {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_msleep"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_msleep (ms : u32) { usleep (u64 :: from (ms) * 1000) ; }
};
}
