// Generated macro for WriteCallback (type)
macro_rules! Depcrate_ioWriteCallback {
() => {
// Module: crate::io
// Provides: {"WriteCallback"}
// Dependencies: {}
pub (crate) type WriteCallback = unsafe extern "C" fn (userdata : * mut c_void , buf : * const u8 , n : size_t , out_n : * mut size_t ,) -> rustls_io_result ;
};
}
