// Generated macro for impl_185 (impl)
macro_rules! Depcrate_streamimpl_185 {
() => {
// Module: crate::stream
// Provides: {"impl_185"}
// Dependencies: {}
impl < S : Stream > Future for LastFuture < S > { type Output = Option < S :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (new) => * this . last = Some (new) , None => return Poll :: Ready (this . last . take ()) , } } } }
};
}
