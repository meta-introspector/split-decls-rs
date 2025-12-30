// Generated macro for sys_truncate (function)
macro_rules! Depcrate_syscallssys_truncate {
() => {
// Module: crate::syscalls
// Provides: {"sys_truncate"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_truncate (path : * const c_char , size : usize) -> i32 { let Ok (path) = unsafe { CStr :: from_ptr (path) } . to_str () else { return - i32 :: from (Errno :: Inval) ; } ; fs :: truncate (path , size) . map_or_else (| e | - i32 :: from (e) , | () | 0) }
};
}
