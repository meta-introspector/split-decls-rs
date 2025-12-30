// Generated macro for sys_mlockall (function)
macro_rules! Depcrate_syscalls_mmansys_mlockall {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_mlockall"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_mlockall (_flags : c_int) -> i32 { 0 }
};
}
