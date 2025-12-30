// Generated macro for sys_write (function)
macro_rules! Depcrate_syscallssys_write {
() => {
// Module: crate::syscalls
// Provides: {"sys_write"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_write (fd : FileDescriptor , buf : * const u8 , len : usize) -> isize { unsafe { write (fd , buf , len) } }
};
}
