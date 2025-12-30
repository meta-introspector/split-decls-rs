// Generated macro for impl_2171 (impl)
macro_rules! Depcrate_io_buf_writerimpl_2171 {
() => {
// Module: crate::io::buf_writer
// Provides: {"impl_2171"}
// Dependencies: {}
impl < W : AsyncRead > AsyncRead for BufWriter < W > { delegate_async_read ! (inner) ; }
};
}
