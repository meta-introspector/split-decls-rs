// Generated macro for sys_dup2 (function)
macro_rules! Depcrate_syscallssys_dup2 {
() => {
// Module: crate::syscalls
// Provides: {"sys_dup2"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_dup2 (fd1 : i32 , fd2 : i32) -> i32 { dup_object2 (fd1 , fd2) . unwrap_or_else (| e | - i32 :: from (e)) }
};
}
