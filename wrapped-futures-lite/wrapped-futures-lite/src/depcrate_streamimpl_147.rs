// Generated macro for impl_147 (impl)
macro_rules! Depcrate_streamimpl_147 {
() => {
// Module: crate::stream
// Provides: {"impl_147"}
// Dependencies: {}
impl < S , P > Stream for Filter < S , P > where S : Stream , P : FnMut (& S :: Item) -> bool , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { None => return Poll :: Ready (None) , Some (v) if (this . predicate) (& v) => return Poll :: Ready (Some (v)) , Some (_) => { } } } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , hi) = self . stream . size_hint () ; (0 , hi) } }
};
}
