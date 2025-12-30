// Generated macro for impl_258 (impl)
macro_rules! Depcrate_session_async_sessionimpl_258 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_258"}
// Dependencies: {}
impl < S > AsyncBufRead for Stream < S > where S : AsyncRead + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Pin :: new (& mut self . get_mut () . stream) . poll_fill_buf (cx) } fn consume (mut self : Pin < & mut Self > , amt : usize) { Pin :: new (& mut self . stream) . consume (amt) ; } }
};
}
