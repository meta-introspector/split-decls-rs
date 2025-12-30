// Generated macro for create_pipe (function)
macro_rules! Depcrate_tests_register_bufferscreate_pipe {
() => {
// Module: crate::tests::register_buffers
// Provides: {"create_pipe"}
// Dependencies: {}
# [doc = " Create a pipe and return both ends as RAII `File` handles"] fn create_pipe () -> io :: Result < (File , File) > { let mut fds = [- 1 , - 1] ; unsafe { if libc :: pipe (fds . as_mut_ptr ()) == - 1 { Err (Error :: last_os_error ()) } else { Ok ((File :: from_raw_fd (fds [0]) , File :: from_raw_fd (fds [1]))) } } }
};
}
