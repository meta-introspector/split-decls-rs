// Generated macro for impl_191 (impl)
macro_rules! Depcrate_streamimpl_191 {
() => {
// Module: crate::stream
// Provides: {"impl_191"}
// Dependencies: {}
impl < S , B , F > Future for FindMapFuture < '_ , S , F > where S : Stream + Unpin + ? Sized , F : FnMut (S :: Item) -> Option < B > , { type Output = Option < B > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => { if let Some (v) = (& mut self . f) (v) { return Poll :: Ready (Some (v)) ; } } None => return Poll :: Ready (None) , } } } }
};
}
