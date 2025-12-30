// Generated macro for impl_982 (impl)
macro_rules! Depcrate_stream_stream_chunksimpl_982 {
() => {
// Module: crate::stream::stream::chunks
// Provides: {"impl_982"}
// Dependencies: {}
impl < St : Stream > Stream for Chunks < St > { type Item = Vec < St :: Item > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (item) => { this . items . push (item) ; if this . items . len () >= * this . cap { return Poll :: Ready (Some (self . take ())) ; } } None => { let last = if this . items . is_empty () { None } else { let full_buf = mem :: take (this . items) ; Some (full_buf) } ; return Poll :: Ready (last) ; } } } } fn size_hint (& self) -> (usize , Option < usize >) { let chunk_len = usize :: from (! self . items . is_empty ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = (lower / self . cap) . saturating_add (chunk_len) ; let upper = match upper { Some (x) => x . checked_add (chunk_len) , None => None , } ; (lower , upper) } }
};
}
