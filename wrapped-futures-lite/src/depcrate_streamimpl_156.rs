// Generated macro for impl_156 (impl)
macro_rules! Depcrate_streamimpl_156 {
() => {
// Module: crate::stream
// Provides: {"impl_156"}
// Dependencies: {}
impl < S , F , T > Stream for FilterMap < S , F > where S : Stream , F : FnMut (S :: Item) -> Option < T > , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { None => return Poll :: Ready (None) , Some (v) => { if let Some (t) = (this . f) (v) { return Poll :: Ready (Some (t)) ; } } } } } }
};
}
