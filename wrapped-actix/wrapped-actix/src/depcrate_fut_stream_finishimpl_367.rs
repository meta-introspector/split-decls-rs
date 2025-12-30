// Generated macro for impl_367 (impl)
macro_rules! Depcrate_fut_stream_finishimpl_367 {
() => {
// Module: crate::fut::stream::finish
// Provides: {"impl_367"}
// Dependencies: {}
impl < S , A > ActorFuture < A > for Finish < S > where S : ActorStream < A > , A : Actor , { type Output = () ; fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < () > { let mut this = self . as_mut () . project () ; while ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) . is_some () { } Poll :: Ready (()) } }
};
}
