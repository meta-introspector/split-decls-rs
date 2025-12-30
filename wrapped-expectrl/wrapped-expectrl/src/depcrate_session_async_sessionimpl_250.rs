// Generated macro for impl_250 (impl)
macro_rules! Depcrate_session_async_sessionimpl_250 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_250"}
// Dependencies: {}
impl < P , S > AsyncRead for Session < P , S > where P : Unpin , S : AsyncRead + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . stream) . poll_read (cx , buf) } }
};
}
