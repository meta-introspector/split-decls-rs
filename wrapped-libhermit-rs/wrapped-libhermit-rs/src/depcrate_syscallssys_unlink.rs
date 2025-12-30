// Generated macro for sys_unlink (function)
macro_rules! Depcrate_syscallssys_unlink {
() => {
// Module: crate::syscalls
// Provides: {"sys_unlink"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_unlink (name : * const c_char) -> i32 { let name = unsafe { CStr :: from_ptr (name) } . to_str () . unwrap () ; fs :: unlink (name) . map_or_else (| e | - i32 :: from (e) , | () | 0) }
};
}
