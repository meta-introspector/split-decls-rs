// Generated macro for impl_188 (impl)
macro_rules! Depcrate_streamimpl_188 {
() => {
// Module: crate::stream
// Provides: {"impl_188"}
// Dependencies: {}
impl < S , P > Future for FindFuture < '_ , S , P > where S : Stream + Unpin + ? Sized , P : FnMut (& S :: Item) -> bool , { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) if (& mut self . predicate) (& v) => return Poll :: Ready (Some (v)) , Some (_) => { } None => return Poll :: Ready (None) , } } } }
};
}
