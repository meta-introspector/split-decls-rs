// Generated macro for impl_257 (impl)
macro_rules! Depcrate_session_async_sessionimpl_257 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_257"}
// Dependencies: {}
impl < S > AsyncRead for Stream < S > where S : AsyncRead + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . stream) . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { for b in bufs { if ! b . is_empty () { return self . poll_read (cx , b) ; } } self . poll_read (cx , & mut []) } }
};
}
