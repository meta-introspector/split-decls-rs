// Generated macro for sys_chdir (function)
macro_rules! Depcrate_syscallssys_chdir {
() => {
// Module: crate::syscalls
// Provides: {"sys_chdir"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_chdir (path : * mut c_char) -> i32 { if let Ok (name) = unsafe { CStr :: from_ptr (path) } . to_str () { crate :: fs :: set_cwd (name) . map (| () | 0) . unwrap_or_else (| e | - i32 :: from (e)) } else { - i32 :: from (Errno :: Inval) } }
};
}
