// Generated macro for impl_124 (impl)
macro_rules! Depcrate_streamimpl_124 {
() => {
// Module: crate::stream
// Provides: {"impl_124"}
// Dependencies: {}
impl < S , C > Future for CollectFuture < S , C > where S : Stream , C : Default + Extend < S :: Item > , { type Output = C ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < C > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => this . collection . extend (Some (e)) , None => return Poll :: Ready (mem :: take (self . project () . collection)) , } } } }
};
}
