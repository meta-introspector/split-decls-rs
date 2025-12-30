// Generated macro for VectoredWriteCallback (type)
macro_rules! Depcrate_ioVectoredWriteCallback {
() => {
// Module: crate::io
// Provides: {"VectoredWriteCallback"}
// Dependencies: {}
pub (crate) type VectoredWriteCallback = unsafe extern "C" fn (userdata : * mut c_void , iov : * const rustls_iovec , count : size_t , out_n : * mut size_t ,) -> rustls_io_result ;
};
}
