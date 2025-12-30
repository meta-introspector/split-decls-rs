// Generated macro for rustls_write_vectored_callback (type)
macro_rules! Depcrate_iorustls_write_vectored_callback {
() => {
// Module: crate::io
// Provides: {"rustls_write_vectored_callback"}
// Dependencies: {}
# [doc = " A callback for `rustls_connection_write_tls_vectored`."] # [doc = ""] # [doc = " An implementation of this callback should attempt to write the bytes in"] # [doc = " the given `count` iovecs to the network."] # [doc = ""] # [doc = " If any bytes were written, the implementation should set out_n to the number of"] # [doc = " bytes written and return 0."] # [doc = ""] # [doc = " If there was an error, the implementation should return a nonzero rustls_io_result,"] # [doc = " which will be passed through to the caller."] # [doc = ""] # [doc = " On POSIX systems, returning `errno` is convenient."] # [doc = ""] # [doc = " On other systems, any appropriate error code works."] # [doc = ""] # [doc = " It's best to make one write attempt to the network per call. Additional write will"] # [doc = " be triggered by subsequent calls to one of the `_write_tls` methods."] # [doc = ""] # [doc = " `userdata` is set to the value provided to `rustls_*_session_set_userdata`. In most"] # [doc = " cases that should be a struct that contains, at a minimum, a file descriptor."] # [doc = ""] # [doc = " The iov and out_n pointers are borrowed and should not be retained across calls."] pub type rustls_write_vectored_callback = Option < unsafe extern "C" fn (userdata : * mut c_void , iov : * const rustls_iovec , count : size_t , out_n : * mut size_t ,) -> rustls_io_result , > ;
};
}
