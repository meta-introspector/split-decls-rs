// Generated macro for sys_ftruncate (function)
macro_rules! Depcrate_syscallssys_ftruncate {
() => {
// Module: crate::syscalls
// Provides: {"sys_ftruncate"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_ftruncate (fd : FileDescriptor , size : usize) -> i32 { fd :: truncate (fd , size) . map_or_else (| e | - i32 :: from (e) , | () | 0) }
};
}
