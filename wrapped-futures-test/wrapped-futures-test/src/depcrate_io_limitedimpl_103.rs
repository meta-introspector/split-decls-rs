// Generated macro for impl_103 (impl)
macro_rules! Depcrate_io_limitedimpl_103 {
() => {
// Module: crate::io::limited
// Provides: {"impl_103"}
// Dependencies: {}
impl < R : AsyncBufRead > AsyncBufRead for Limited < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . project () . io . poll_fill_buf (cx) } fn consume (self : Pin < & mut Self > , amount : usize) { self . project () . io . consume (amount) } }
};
}
