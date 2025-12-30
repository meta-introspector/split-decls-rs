// Generated macro for impl_855 (impl)
macro_rules! Depcrate_serverimpl_855 {
() => {
// Module: crate::server
// Provides: {"impl_855"}
// Dependencies: {}
# [cfg (feature = "stream")] impl < T , B > futures_core :: Stream for Connection < T , B > where T : AsyncRead + AsyncWrite + Unpin , B : Buf , { type Item = Result < (Request < RecvStream > , SendResponse < B >) , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_accept (cx) } }
};
}
