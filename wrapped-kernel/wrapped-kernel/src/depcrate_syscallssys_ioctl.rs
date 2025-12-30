// Generated macro for sys_ioctl (function)
macro_rules! Depcrate_syscallssys_ioctl {
() => {
// Module: crate::syscalls
// Provides: {"sys_ioctl"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_ioctl (fd : FileDescriptor , cmd : i32 , argp : * mut core :: ffi :: c_void ,) -> i32 { const FIONBIO : i32 = 0x8008_667eu32 as i32 ; if cmd == FIONBIO { let value = unsafe { * (argp as * const i32) } ; let status_flags = if value != 0 { fd :: StatusFlags :: O_NONBLOCK } else { fd :: StatusFlags :: empty () } ; let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . write () . await . set_status_flags (status_flags) . await } , None ,) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) } else { - i32 :: from (Errno :: Inval) } }
};
}
