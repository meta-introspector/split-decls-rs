// Generated macro for impl_251 (impl)
macro_rules! Depcrate_session_async_sessionimpl_251 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_251"}
// Dependencies: {}
impl < P , S > AsyncBufRead for Session < P , S > where P : Unpin , S : AsyncRead + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Pin :: new (& mut self . get_mut () . stream) . poll_fill_buf (cx) } fn consume (mut self : Pin < & mut Self > , amt : usize) { Pin :: new (& mut self . stream) . consume (amt) ; } }
};
}
