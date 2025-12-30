// Generated macro for sys_dup (function)
macro_rules! Depcrate_syscallssys_dup {
() => {
// Module: crate::syscalls
// Provides: {"sys_dup"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_dup (fd : i32) -> i32 { dup_object (fd) . unwrap_or_else (| e | - i32 :: from (e)) }
};
}
