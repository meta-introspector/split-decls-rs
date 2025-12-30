// Generated macro for impl_2150 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2150 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2150"}
// Dependencies: {}
impl < R : AsyncWrite > AsyncWrite for BufReader < R > { delegate_async_write ! (inner) ; }
};
}
