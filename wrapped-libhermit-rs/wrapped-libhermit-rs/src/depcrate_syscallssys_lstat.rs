// Generated macro for sys_lstat (function)
macro_rules! Depcrate_syscallssys_lstat {
() => {
// Module: crate::syscalls
// Provides: {"sys_lstat"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_lstat (name : * const c_char , stat : * mut FileAttr) -> i32 { let name = unsafe { CStr :: from_ptr (name) } . to_str () . unwrap () ; match fs :: read_lstat (name) { Ok (attr) => unsafe { * stat = attr ; 0 } , Err (e) => - i32 :: from (e) , } }
};
}
