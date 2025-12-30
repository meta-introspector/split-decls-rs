// Generated macro for sys_access (function)
macro_rules! Depcrate_syscallssys_access {
() => {
// Module: crate::syscalls
// Provides: {"sys_access"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_access (name : * const c_char , flags : i32) -> i32 { let Some (access_option) = AccessOption :: from_bits (flags) else { return - i32 :: from (Errno :: Inval) ; } ; if access_option . contains (AccessOption :: F_OK) && access_option != AccessOption :: F_OK { return - i32 :: from (Errno :: Inval) ; } let Ok (name) = unsafe { CStr :: from_ptr (name) } . to_str () else { return - i32 :: from (Errno :: Inval) ; } ; match crate :: fs :: read_lstat (name) { Err (e) => - i32 :: from (e) , Ok (stat) if access_option . can_access (stat . st_mode) => 0 , Ok (_) => - i32 :: from (Errno :: Acces) , } }
};
}
