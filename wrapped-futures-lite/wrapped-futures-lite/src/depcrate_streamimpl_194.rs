// Generated macro for impl_194 (impl)
macro_rules! Depcrate_streamimpl_194 {
() => {
// Module: crate::stream
// Provides: {"impl_194"}
// Dependencies: {}
impl < S , P > Future for PositionFuture < '_ , S , P > where S : Stream + Unpin + ? Sized , P : FnMut (S :: Item) -> bool , { type Output = Option < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => { if (& mut self . predicate) (v) { return Poll :: Ready (Some (self . index)) ; } else { self . index += 1 ; } } None => return Poll :: Ready (None) , } } } }
};
}
