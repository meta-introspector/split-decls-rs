// Generated macro for sys_close (function)
macro_rules! Depcrate_syscallssys_close {
() => {
// Module: crate::syscalls
// Provides: {"sys_close"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_close (fd : FileDescriptor) -> i32 { let obj = remove_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | _ | 0) }
};
}
