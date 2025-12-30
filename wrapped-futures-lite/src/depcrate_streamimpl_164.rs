// Generated macro for impl_164 (impl)
macro_rules! Depcrate_streamimpl_164 {
() => {
// Module: crate::stream
// Provides: {"impl_164"}
// Dependencies: {}
impl < S : Stream > Stream for Skip < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => match * this . n { 0 => return Poll :: Ready (Some (v)) , _ => * this . n -= 1 , } , None => return Poll :: Ready (None) , } } } }
};
}
