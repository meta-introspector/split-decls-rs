// Generated macro for impl_139 (impl)
macro_rules! Depcrate_streamimpl_139 {
() => {
// Module: crate::stream
// Provides: {"impl_139"}
// Dependencies: {}
impl < S , F , T > Stream for Map < S , F > where S : Stream , F : FnMut (S :: Item) -> T , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; Poll :: Ready (next . map (this . f)) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
