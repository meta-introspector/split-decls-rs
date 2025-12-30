// Generated macro for sys_opendir (function)
macro_rules! Depcrate_syscallssys_opendir {
() => {
// Module: crate::syscalls
// Provides: {"sys_opendir"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_opendir (name : * const c_char) -> FileDescriptor { if let Ok (name) = unsafe { CStr :: from_ptr (name) } . to_str () { crate :: fs :: opendir (name) . unwrap_or_else (| e | - i32 :: from (e)) } else { - i32 :: from (Errno :: Inval) } }
};
}
