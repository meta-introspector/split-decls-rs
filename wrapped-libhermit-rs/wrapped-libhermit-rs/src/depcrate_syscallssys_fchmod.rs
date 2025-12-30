// Generated macro for sys_fchmod (function)
macro_rules! Depcrate_syscallssys_fchmod {
() => {
// Module: crate::syscalls
// Provides: {"sys_fchmod"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_fchmod (fd : FileDescriptor , mode : u32) -> i32 { let Some (access_permission) = AccessPermission :: from_bits (mode) else { return - i32 :: from (Errno :: Inval) ; } ; crate :: fd :: chmod (fd , access_permission) . map (| () | 0) . unwrap_or_else (| e | - i32 :: from (e)) }
};
}
