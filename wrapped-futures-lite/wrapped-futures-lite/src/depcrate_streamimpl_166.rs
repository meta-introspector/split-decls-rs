// Generated macro for impl_166 (impl)
macro_rules! Depcrate_streamimpl_166 {
() => {
// Module: crate::stream
// Provides: {"impl_166"}
// Dependencies: {}
impl < S , P > Stream for SkipWhile < S , P > where S : Stream , P : FnMut (& S :: Item) -> bool , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => match this . predicate { Some (p) => { if ! p (& v) { * this . predicate = None ; return Poll :: Ready (Some (v)) ; } } None => return Poll :: Ready (Some (v)) , } , None => return Poll :: Ready (None) , } } } }
};
}
