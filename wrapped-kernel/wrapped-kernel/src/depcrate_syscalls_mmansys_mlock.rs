// Generated macro for sys_mlock (function)
macro_rules! Depcrate_syscalls_mmansys_mlock {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_mlock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_mlock (_addr : * const c_void , _size : usize) -> i32 { 0 }
};
}
