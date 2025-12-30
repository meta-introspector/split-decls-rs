// Generated macro for impl_359 (impl)
macro_rules! Depcrate_fut_stream_collectimpl_359 {
() => {
// Module: crate::fut::stream::collect
// Provides: {"impl_359"}
// Dependencies: {}
impl < S , A , C > ActorFuture < A > for Collect < S , C > where S : ActorStream < A > , A : Actor , C : Default + Extend < S :: Item > , { type Output = C ; fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) { Some (e) => this . collection . extend (Some (e)) , None => return Poll :: Ready (mem :: take (this . collection)) , } } } }
};
}
