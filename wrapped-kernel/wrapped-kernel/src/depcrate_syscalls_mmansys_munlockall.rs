// Generated macro for sys_munlockall (function)
macro_rules! Depcrate_syscalls_mmansys_munlockall {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_munlockall"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_munlockall (_flags : c_int) -> i32 { 0 }
};
}
