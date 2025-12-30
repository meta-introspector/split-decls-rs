// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_stream_stream_buffer_unorderedimpl_1031 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"impl_1031"}
// Dependencies: {}
impl < St > Stream for BufferUnordered < St > where St : Stream , St :: Item : Future , { type Item = < St :: Item as Future > :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; while this . max . map (| max | this . in_progress_queue . len () < max . get ()) . unwrap_or (true) { match this . stream . as_mut () . poll_next (cx) { Poll :: Ready (Some (fut)) => this . in_progress_queue . push (fut) , Poll :: Ready (None) | Poll :: Pending => break , } } match this . in_progress_queue . poll_next_unpin (cx) { x @ Poll :: Pending | x @ Poll :: Ready (Some (_)) => return x , Poll :: Ready (None) => { } } if this . stream . is_done () { Poll :: Ready (None) } else { Poll :: Pending } } fn size_hint (& self) -> (usize , Option < usize >) { let queue_len = self . in_progress_queue . len () ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (queue_len) ; let upper = match upper { Some (x) => x . checked_add (queue_len) , None => None , } ; (lower , upper) } }
};
}
