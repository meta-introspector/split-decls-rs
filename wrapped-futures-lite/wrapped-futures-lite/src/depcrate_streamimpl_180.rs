// Generated macro for impl_180 (impl)
macro_rules! Depcrate_streamimpl_180 {
() => {
// Module: crate::stream
// Provides: {"impl_180"}
// Dependencies: {}
impl < S , F > Stream for Inspect < S , F > where S : Stream , F : FnMut (& S :: Item) , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let next = ready ! (this . stream . as_mut () . poll_next (cx)) ; if let Some (x) = & next { (this . f) (x) ; } Poll :: Ready (next) } }
};
}
