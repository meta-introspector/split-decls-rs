// Generated macro for impl_238 (impl)
macro_rules! Depcrate_replimpl_238 {
() => {
// Module: crate::repl
// Provides: {"impl_238"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > AsyncRead for ReplSession < S > where S : AsyncRead + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { S :: poll_read (Pin :: new (self . get_session_mut ()) , cx , buf) } }
};
}
