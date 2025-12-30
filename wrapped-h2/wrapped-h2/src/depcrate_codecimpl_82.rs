// Generated macro for impl_82 (impl)
macro_rules! Depcrate_codecimpl_82 {
() => {
// Module: crate::codec
// Provides: {"impl_82"}
// Dependencies: {}
impl < T , B > Stream for Codec < T , B > where T : AsyncRead + Unpin , { type Item = Result < Frame , Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut self . inner) . poll_next (cx) } }
};
}
