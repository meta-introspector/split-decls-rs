// Generated macro for sys_munlock (function)
macro_rules! Depcrate_syscalls_mmansys_munlock {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_munlock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_munlock (_addr : * const c_void , _size : usize) -> i32 { 0 }
};
}
