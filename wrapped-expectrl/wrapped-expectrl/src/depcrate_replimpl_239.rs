// Generated macro for impl_239 (impl)
macro_rules! Depcrate_replimpl_239 {
() => {
// Module: crate::repl
// Provides: {"impl_239"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > AsyncBufRead for ReplSession < S > where S : AsyncBufRead + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { S :: poll_fill_buf (Pin :: new (self . get_mut () . get_session_mut ()) , cx) } fn consume (mut self : Pin < & mut Self > , amt : usize) { S :: consume (Pin :: new (self . get_session_mut ()) , amt) } }
};
}
