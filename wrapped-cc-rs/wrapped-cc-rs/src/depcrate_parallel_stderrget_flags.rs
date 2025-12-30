// Generated macro for get_flags (function)
macro_rules! Depcrate_parallel_stderrget_flags {
() => {
// Module: crate::parallel::stderr
// Provides: {"get_flags"}
// Dependencies: {}
# [cfg (unix)] fn get_flags (fd : std :: os :: unix :: io :: RawFd) -> Result < i32 , Error > { let flags = unsafe { libc :: fcntl (fd , libc :: F_GETFL , 0) } ; if flags == - 1 { Err (Error :: new (ErrorKind :: IOError , format ! ("Failed to get flags for pipe {}: {}" , fd , std :: io :: Error :: last_os_error ()) ,)) } else { Ok (flags) } }
};
}
