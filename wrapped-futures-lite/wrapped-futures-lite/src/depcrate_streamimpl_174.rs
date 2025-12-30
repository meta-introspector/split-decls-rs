// Generated macro for impl_174 (impl)
macro_rules! Depcrate_streamimpl_174 {
() => {
// Module: crate::stream
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'a , S , T : 'a > Stream for Copied < S > where S : Stream < Item = & 'a T > , T : Copy , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; Poll :: Ready (next . copied ()) } }
};
}
