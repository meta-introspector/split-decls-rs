// Generated macro for impl_134 (impl)
macro_rules! Depcrate_zlib_stream_deflateimpl_134 {
() => {
// Module: crate::zlib::stream::deflate
// Provides: {"impl_134"}
// Dependencies: {}
impl Drop for Compress { fn drop (& mut self) { unsafe { libz_rs_sys :: deflateEnd (& mut self . 0) } ; } }
};
}
