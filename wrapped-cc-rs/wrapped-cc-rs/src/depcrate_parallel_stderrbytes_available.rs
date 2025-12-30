// Generated macro for bytes_available (function)
macro_rules! Depcrate_parallel_stderrbytes_available {
() => {
// Module: crate::parallel::stderr
// Provides: {"bytes_available"}
// Dependencies: {}
pub fn bytes_available (stderr : & mut ChildStderr) -> Result < usize , Error > { let mut bytes_available = 0 ; # [cfg (windows)] { use :: find_msvc_tools :: windows_sys :: PeekNamedPipe ; use std :: os :: windows :: io :: AsRawHandle ; use std :: ptr :: null_mut ; if unsafe { PeekNamedPipe (stderr . as_raw_handle () , null_mut () , 0 , null_mut () , & mut bytes_available , null_mut () ,) } == 0 { return Err (Error :: new (ErrorKind :: IOError , format ! ("PeekNamedPipe failed with {}" , std :: io :: Error :: last_os_error ()) ,)) ; } } # [cfg (unix)] { use std :: os :: unix :: io :: AsRawFd ; if unsafe { libc :: ioctl (stderr . as_raw_fd () , libc :: FIONREAD , & mut bytes_available) } != 0 { return Err (Error :: new (ErrorKind :: IOError , format ! ("ioctl failed with {}" , std :: io :: Error :: last_os_error ()) ,)) ; } } Ok (bytes_available . try_into () . unwrap ()) }
};
}
