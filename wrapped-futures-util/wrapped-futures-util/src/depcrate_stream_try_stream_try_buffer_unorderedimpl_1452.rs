// Generated macro for impl_1452 (impl)
macro_rules! Depcrate_stream_try_stream_try_buffer_unorderedimpl_1452 {
() => {
// Module: crate::stream::try_stream::try_buffer_unordered
// Provides: {"impl_1452"}
// Dependencies: {}
impl < St > Stream for TryBufferUnordered < St > where St : TryStream , St :: Ok : TryFuture < Error = St :: Error > , { type Item = Result < < St :: Ok as TryFuture > :: Ok , St :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; while this . max . map (| max | this . in_progress_queue . len () < max . get ()) . unwrap_or (true) { match this . stream . as_mut () . poll_next (cx) ? { Poll :: Ready (Some (fut)) => this . in_progress_queue . push (fut . into_future ()) , Poll :: Ready (None) | Poll :: Pending => break , } } match this . in_progress_queue . poll_next_unpin (cx) { x @ Poll :: Pending | x @ Poll :: Ready (Some (_)) => return x , Poll :: Ready (None) => { } } if this . stream . is_done () { Poll :: Ready (None) } else { Poll :: Pending } } }
};
}
