// Generated macro for sys_open (function)
macro_rules! Depcrate_syscallssys_open {
() => {
// Module: crate::syscalls
// Provides: {"sys_open"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_open (name : * const c_char , flags : i32 , mode : u32) -> FileDescriptor { let Some (flags) = OpenOption :: from_bits (flags) else { return - i32 :: from (Errno :: Inval) ; } ; let Some (mode) = AccessPermission :: from_bits (mode) else { return - i32 :: from (Errno :: Inval) ; } ; if let Ok (name) = unsafe { CStr :: from_ptr (name) } . to_str () { crate :: fs :: open (name , flags , mode) . unwrap_or_else (| e | - i32 :: from (e)) } else { - i32 :: from (Errno :: Inval) } }
};
}
