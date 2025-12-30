// Generated macro for sys_lseek (function)
macro_rules! Depcrate_syscallssys_lseek {
() => {
// Module: crate::syscalls
// Provides: {"sys_lseek"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_lseek (fd : FileDescriptor , offset : isize , whence : i32) -> isize { let whence = u8 :: try_from (whence) . unwrap () ; let whence = SeekWhence :: try_from (whence) . unwrap () ; crate :: fd :: lseek (fd , offset , whence) . unwrap_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap ()) }
};
}
