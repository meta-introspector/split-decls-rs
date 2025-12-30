// Generated macro for sys_faccessat (function)
macro_rules! Depcrate_syscallssys_faccessat {
() => {
// Module: crate::syscalls
// Provides: {"sys_faccessat"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_faccessat (dirfd : FileDescriptor , name : * const c_char , _mode : i32 , flags : i32 ,) -> i32 { let Some (access_option) = AccessOption :: from_bits (flags) else { return - i32 :: from (Errno :: Inval) ; } ; let Ok (name) = unsafe { CStr :: from_ptr (name) } . to_str () else { return - i32 :: from (Errno :: Inval) ; } ; const AT_SYMLINK_NOFOLLOW : i32 = 0x100 ; const AT_FDCWD : i32 = - 100 ; let stat = if name . starts_with ("/") || dirfd == AT_FDCWD { let no_follow : bool = (flags & AT_SYMLINK_NOFOLLOW) != 0 ; if no_follow { fs :: read_stat (name) } else { fs :: read_lstat (name) } } else { warn ! ("faccessat with directory relative to fd is not implemented!") ; return - i32 :: from (Errno :: Nosys) ; } ; match stat { Err (e) => - i32 :: from (e) , Ok (stat) if access_option . can_access (stat . st_mode) => 0 , Ok (_) => - i32 :: from (Errno :: Acces) , } }
};
}
