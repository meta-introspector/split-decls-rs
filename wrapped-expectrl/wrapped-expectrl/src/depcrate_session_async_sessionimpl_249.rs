// Generated macro for impl_249 (impl)
macro_rules! Depcrate_session_async_sessionimpl_249 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_249"}
// Dependencies: {}
impl < P , S > AsyncWrite for Session < P , S > where P : Unpin , S : AsyncWrite + Unpin , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . get_mut () . stream) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . stream) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . stream) . poll_close (cx) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . stream) . poll_write_vectored (cx , bufs) } }
};
}
