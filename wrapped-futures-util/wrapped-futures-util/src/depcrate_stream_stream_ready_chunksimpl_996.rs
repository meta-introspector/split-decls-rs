// Generated macro for impl_996 (impl)
macro_rules! Depcrate_stream_stream_ready_chunksimpl_996 {
() => {
// Module: crate::stream::stream::ready_chunks
// Provides: {"impl_996"}
// Dependencies: {}
impl < St : Stream > Stream for ReadyChunks < St > { type Item = Vec < St :: Item > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let mut items : Vec < St :: Item > = Vec :: new () ; loop { match this . stream . as_mut () . poll_next (cx) { Poll :: Pending => { return if items . is_empty () { Poll :: Pending } else { Poll :: Ready (Some (items)) } } Poll :: Ready (Some (item)) => { if items . is_empty () { items . reserve (* this . cap) ; } items . push (item) ; if items . len () >= * this . cap { return Poll :: Ready (Some (items)) ; } } Poll :: Ready (None) => { let last = if items . is_empty () { None } else { Some (items) } ; return Poll :: Ready (last) ; } } } } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . stream . size_hint () ; let lower = lower / self . cap ; (lower , upper) } }
};
}
