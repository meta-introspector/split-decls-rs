// Generated macro for impl_256 (impl)
macro_rules! Depcrate_session_async_sessionimpl_256 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_256"}
// Dependencies: {}
impl < S > AsyncWrite for Stream < S > where S : AsyncWrite + Unpin , { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut * self . stream . get_mut ()) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut * self . stream . get_mut ()) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut * self . stream . get_mut ()) . poll_close (cx) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut * self . stream . get_mut ()) . poll_write_vectored (cx , bufs) } }
};
}
