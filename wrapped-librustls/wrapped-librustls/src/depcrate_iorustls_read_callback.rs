// Generated macro for rustls_read_callback (type)
macro_rules! Depcrate_iorustls_read_callback {
() => {
// Module: crate::io
// Provides: {"rustls_read_callback"}
// Dependencies: {}
# [doc = " A callback for `rustls_connection_read_tls`."] # [doc = ""] # [doc = " An implementation of this callback should attempt to read up to n bytes from the"] # [doc = " network, storing them in `buf`. If any bytes were stored, the implementation should"] # [doc = " set out_n to the number of bytes stored and return 0."] # [doc = ""] # [doc = " If there was an error, the implementation should return a nonzero rustls_io_result,"] # [doc = " which will be passed through to the caller."] # [doc = ""] # [doc = " On POSIX systems, returning `errno` is convenient."] # [doc = ""] # [doc = " On other systems, any appropriate error code works."] # [doc = ""] # [doc = " It's best to make one read attempt to the network per call. Additional reads will"] # [doc = " be triggered by subsequent calls to one of the `_read_tls` methods."] # [doc = ""] # [doc = " `userdata` is set to the value provided to `rustls_connection_set_userdata`."] # [doc = " In most cases that should be a struct that contains, at a minimum, a file descriptor."] # [doc = ""] # [doc = " The buf and out_n pointers are borrowed and should not be retained across calls."] pub type rustls_read_callback = Option < unsafe extern "C" fn (userdata : * mut c_void , buf : * mut u8 , n : size_t , out_n : * mut size_t ,) -> rustls_io_result , > ;
};
}
