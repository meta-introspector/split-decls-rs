// Generated macro for impl_2172 (impl)
macro_rules! Depcrate_io_buf_writerimpl_2172 {
() => {
// Module: crate::io::buf_writer
// Provides: {"impl_2172"}
// Dependencies: {}
impl < W : AsyncBufRead > AsyncBufRead for BufWriter < W > { delegate_async_buf_read ! (inner) ; }
};
}
