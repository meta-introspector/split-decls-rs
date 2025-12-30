// Generated macro for set_flags (function)
macro_rules! Depcrate_parallel_stderrset_flags {
() => {
// Module: crate::parallel::stderr
// Provides: {"set_flags"}
// Dependencies: {}
# [cfg (unix)] fn set_flags (fd : std :: os :: unix :: io :: RawFd , flags : std :: os :: raw :: c_int) -> Result < () , Error > { if unsafe { libc :: fcntl (fd , libc :: F_SETFL , flags) } == - 1 { Err (Error :: new (ErrorKind :: IOError , format ! ("Failed to set flags for pipe {}: {}" , fd , std :: io :: Error :: last_os_error ()) ,)) } else { Ok (()) } }
};
}
