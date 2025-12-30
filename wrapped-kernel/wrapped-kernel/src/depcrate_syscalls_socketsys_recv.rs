// Generated macro for sys_recv (function)
macro_rules! Depcrate_syscalls_socketsys_recv {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_recv"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_recv (fd : i32 , buf : * mut u8 , len : usize , flags : i32) -> isize { if flags == 0 { let slice = unsafe { core :: slice :: from_raw_parts_mut (buf . cast () , len) } ; fd :: read (fd , slice) . map_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap () , | v | v . try_into () . unwrap () ,) } else { (- i32 :: from (Errno :: Inval)) . try_into () . unwrap () } }
};
}
