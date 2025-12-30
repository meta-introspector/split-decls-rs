// Generated macro for sys_setprio (function)
macro_rules! Depcrate_syscalls_taskssys_setprio {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_setprio"}
// Dependencies: {}
# [cfg (feature = "newlib")] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_setprio (_id : * const Tid , _prio : i32) -> i32 { - i32 :: from (Errno :: Nosys) }
};
}
