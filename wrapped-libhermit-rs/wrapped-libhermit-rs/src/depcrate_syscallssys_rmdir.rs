// Generated macro for sys_rmdir (function)
macro_rules! Depcrate_syscallssys_rmdir {
() => {
// Module: crate::syscalls
// Provides: {"sys_rmdir"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_rmdir (name : * const c_char) -> i32 { let name = unsafe { CStr :: from_ptr (name) } . to_str () . unwrap () ; crate :: fs :: remove_dir (name) . map_or_else (| e | - i32 :: from (e) , | () | 0) }
};
}
