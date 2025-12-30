// Generated macro for macro_271 (macro)
macro_rules! Depcrate_unistdmacro_271 {
() => {
// Module: crate::unistd
// Provides: {"macro_271"}
// Dependencies: {}
feature ! { #! [feature = "process"] # [doc = " Get the group id of the calling process (see"] # [doc = "[getpgrp(3)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpgrp.html))."] # [doc = ""] # [doc = " Get the process group id (PGID) of the calling process."] # [doc = " According to the man page it is always successful."] # [inline] pub fn getpgrp () -> Pid { Pid (unsafe { libc :: getpgrp () }) } # [doc = " Get the caller's thread ID (see"] # [doc = " [gettid(2)](https://man7.org/linux/man-pages/man2/gettid.2.html)."] # [doc = ""] # [doc = " This function is only available on Linux based systems.  In a single"] # [doc = " threaded process, the main thread will have the same ID as the process.  In"] # [doc = " a multithreaded process, each thread will have a unique thread id but the"] # [doc = " same process ID."] # [doc = ""] # [doc = " No error handling is required as a thread id should always exist for any"] # [doc = " process, even if threads are not being used."] # [cfg (linux_android)] # [inline] pub fn gettid () -> Pid { Pid (unsafe { libc :: syscall (libc :: SYS_gettid) as pid_t }) } }
};
}
