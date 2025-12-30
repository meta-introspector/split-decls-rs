// Generated macro for sys_kill (function)
macro_rules! Depcrate_syscalls_taskssys_kill {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_kill"}
// Dependencies: {}
# [cfg (feature = "newlib")] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_kill (dest : Tid , signum : i32) -> i32 { debug ! ("sys_kill is unimplemented, returning -ENOSYS for killing {dest} with signal {signum}") ; - i32 :: from (Errno :: Nosys) }
};
}
