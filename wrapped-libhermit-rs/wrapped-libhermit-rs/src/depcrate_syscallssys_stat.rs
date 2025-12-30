// Generated macro for sys_stat (function)
macro_rules! Depcrate_syscallssys_stat {
() => {
// Module: crate::syscalls
// Provides: {"sys_stat"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_stat (name : * const c_char , stat : * mut FileAttr) -> i32 { let name = unsafe { CStr :: from_ptr (name) } . to_str () . unwrap () ; match fs :: read_stat (name) { Ok (attr) => unsafe { * stat = attr ; 0 } , Err (e) => - i32 :: from (e) , } }
};
}
