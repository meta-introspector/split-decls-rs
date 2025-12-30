// Generated macro for impl_172 (impl)
macro_rules! Depcrate_streamimpl_172 {
() => {
// Module: crate::stream
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'a , S , T : 'a > Stream for Cloned < S > where S : Stream < Item = & 'a T > , T : Clone , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; Poll :: Ready (next . cloned ()) } }
};
}
