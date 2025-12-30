// Generated macro for sys_read (function)
macro_rules! Depcrate_syscallssys_read {
() => {
// Module: crate::syscalls
// Provides: {"sys_read"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_read (fd : FileDescriptor , buf : * mut u8 , len : usize) -> isize { let slice = unsafe { core :: slice :: from_raw_parts_mut (buf . cast () , len) } ; crate :: fd :: read (fd , slice) . map_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap () , | v | v . try_into () . unwrap () ,) }
};
}
