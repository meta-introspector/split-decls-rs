// Generated macro for impl_1802 (impl)
macro_rules! Depcrate_stream_select_allimpl_1802 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1802"}
// Dependencies: {}
impl < St : Stream + Unpin > Stream for SelectAll < St > { type Item = St :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { match ready ! (self . inner . poll_next_unpin (cx)) { Some ((Some (item) , remaining)) => { self . push (remaining) ; return Poll :: Ready (Some (item)) ; } Some ((None , _)) => { } None => return Poll :: Ready (None) , } } } }
};
}
