// Generated macro for sys_poll (function)
macro_rules! Depcrate_syscallssys_poll {
() => {
// Module: crate::syscalls
// Provides: {"sys_poll"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_poll (fds : * mut PollFd , nfds : usize , timeout : i32) -> i32 { let slice = unsafe { core :: slice :: from_raw_parts_mut (fds , nfds) } ; let timeout = if timeout >= 0 { Some (core :: time :: Duration :: from_millis (timeout . try_into () . unwrap () ,)) } else { None } ; crate :: fd :: poll (slice , timeout) . map_or_else (| e | { if e == Errno :: Time { 0 } else { - i32 :: from (e) } } , | v | v . try_into () . unwrap () ,) }
};
}
