// Generated macro for impl_747 (impl)
macro_rules! Depcrate_stream_stream_fuseimpl_747 {
() => {
// Module: crate::stream::stream::fuse
// Provides: {"impl_747"}
// Dependencies: {}
impl < S : Stream > Stream for Fuse < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { let this = self . project () ; if * this . done { return Poll :: Ready (None) ; } let item = ready ! (this . stream . poll_next (cx)) ; if item . is_none () { * this . done = true ; } Poll :: Ready (item) } fn size_hint (& self) -> (usize , Option < usize >) { if self . done { (0 , Some (0)) } else { self . stream . size_hint () } } }
};
}
