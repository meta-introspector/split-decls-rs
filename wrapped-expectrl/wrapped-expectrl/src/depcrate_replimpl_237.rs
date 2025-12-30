// Generated macro for impl_237 (impl)
macro_rules! Depcrate_replimpl_237 {
() => {
// Module: crate::repl
// Provides: {"impl_237"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > AsyncWrite for ReplSession < S > where S : AsyncWrite + Unpin , { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { S :: poll_write (Pin :: new (self . get_session_mut ()) , cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { S :: poll_flush (Pin :: new (self . get_session_mut ()) , cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { S :: poll_close (Pin :: new (self . get_session_mut ()) , cx) } }
};
}
