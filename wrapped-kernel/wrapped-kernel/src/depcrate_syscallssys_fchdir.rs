// Generated macro for sys_fchdir (function)
macro_rules! Depcrate_syscallssys_fchdir {
() => {
// Module: crate::syscalls
// Provides: {"sys_fchdir"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_fchdir (_fd : FileDescriptor) -> i32 { - i32 :: from (Errno :: Nosys) }
};
}
