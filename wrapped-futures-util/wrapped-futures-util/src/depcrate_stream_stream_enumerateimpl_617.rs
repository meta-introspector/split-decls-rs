// Generated macro for impl_617 (impl)
macro_rules! Depcrate_stream_stream_enumerateimpl_617 {
() => {
// Module: crate::stream::stream::enumerate
// Provides: {"impl_617"}
// Dependencies: {}
impl < St : Stream > Stream for Enumerate < St > { type Item = (usize , St :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (item) => { let prev_count = * this . count ; * this . count += 1 ; Poll :: Ready (Some ((prev_count , item))) } None => Poll :: Ready (None) , } } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
