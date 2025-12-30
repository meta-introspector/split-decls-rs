// Generated macro for rustls_write_callback (type)
macro_rules! Depcrate_iorustls_write_callback {
() => {
// Module: crate::io
// Provides: {"rustls_write_callback"}
// Dependencies: {}
# [doc = " A callback for `rustls_connection_write_tls` or `rustls_accepted_alert_write_tls`."] # [doc = ""] # [doc = " An implementation of this callback should attempt to write the `n` bytes in buf"] # [doc = " to the network."] # [doc = ""] # [doc = " If any bytes were written, the implementation should set `out_n` to the number of"] # [doc = " bytes stored and return 0."] # [doc = ""] # [doc = " If there was an error, the implementation should return a nonzero `rustls_io_result`,"] # [doc = " which will be passed through to the caller."] # [doc = ""] # [doc = " On POSIX systems, returning `errno` is convenient."] # [doc = ""] # [doc = " On other systems, any appropriate error code works."] # [doc = ""] # [doc = " It's best to make one write attempt to the network per call. Additional writes will"] # [doc = " be triggered by subsequent calls to rustls_connection_write_tls."] # [doc = ""] # [doc = " `userdata` is set to the value provided to `rustls_connection_set_userdata`. In most"] # [doc = " cases that should be a struct that contains, at a minimum, a file descriptor."] # [doc = ""] # [doc = " The buf and out_n pointers are borrowed and should not be retained across calls."] pub type rustls_write_callback = Option < unsafe extern "C" fn (userdata : * mut c_void , buf : * const u8 , n : size_t , out_n : * mut size_t ,) -> rustls_io_result , > ;
};
}
