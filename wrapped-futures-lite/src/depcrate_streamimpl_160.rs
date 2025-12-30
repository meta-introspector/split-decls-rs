// Generated macro for impl_160 (impl)
macro_rules! Depcrate_streamimpl_160 {
() => {
// Module: crate::stream
// Provides: {"impl_160"}
// Dependencies: {}
impl < S , P > Stream for TakeWhile < S , P > where S : Stream , P : FnMut (& S :: Item) -> bool , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (v) => { if (this . predicate) (& v) { Poll :: Ready (Some (v)) } else { Poll :: Ready (None) } } None => Poll :: Ready (None) , } } }
};
}
