// Generated macro for ReadCallback (type)
macro_rules! Depcrate_ioReadCallback {
() => {
// Module: crate::io
// Provides: {"ReadCallback"}
// Dependencies: {}
pub (crate) type ReadCallback = unsafe extern "C" fn (userdata : * mut c_void , buf : * mut u8 , n : size_t , out_n : * mut size_t ,) -> rustls_io_result ;
};
}
