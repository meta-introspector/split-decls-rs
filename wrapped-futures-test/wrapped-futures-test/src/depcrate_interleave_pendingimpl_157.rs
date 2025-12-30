// Generated macro for impl_157 (impl)
macro_rules! Depcrate_interleave_pendingimpl_157 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_157"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for InterleavePending < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { self . poll_with (cx , | r , cx | r . poll_read (cx , buf)) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { self . poll_with (cx , | r , cx | r . poll_read_vectored (cx , bufs)) } }
};
}
