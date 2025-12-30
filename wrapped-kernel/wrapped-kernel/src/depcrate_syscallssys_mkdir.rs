// Generated macro for sys_mkdir (function)
macro_rules! Depcrate_syscallssys_mkdir {
() => {
// Module: crate::syscalls
// Provides: {"sys_mkdir"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_mkdir (name : * const c_char , mode : u32) -> i32 { let name = unsafe { CStr :: from_ptr (name) } . to_str () . unwrap () ; let Some (mode) = AccessPermission :: from_bits (mode) else { return - i32 :: from (Errno :: Inval) ; } ; crate :: fs :: create_dir (name , mode) . map_or_else (| e | - i32 :: from (e) , | () | 0) }
};
}
