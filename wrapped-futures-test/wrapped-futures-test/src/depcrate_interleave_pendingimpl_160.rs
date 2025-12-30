// Generated macro for impl_160 (impl)
macro_rules! Depcrate_interleave_pendingimpl_160 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_160"}
// Dependencies: {}
impl < R : AsyncBufRead > AsyncBufRead for InterleavePending < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . poll_with (cx , R :: poll_fill_buf) } fn consume (self : Pin < & mut Self > , amount : usize) { self . project () . inner . consume (amount) } }
};
}
