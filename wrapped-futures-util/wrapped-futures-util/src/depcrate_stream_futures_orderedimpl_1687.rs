// Generated macro for impl_1687 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1687 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1687"}
// Dependencies: {}
impl < Fut : Future > Stream for FuturesOrdered < Fut > { type Item = Fut :: Output ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = & mut * self ; if let Some (next_output) = this . queued_outputs . peek_mut () { if next_output . index == this . next_outgoing_index { this . next_outgoing_index += 1 ; return Poll :: Ready (Some (PeekMut :: pop (next_output) . data)) ; } } loop { match ready ! (this . in_progress_queue . poll_next_unpin (cx)) { Some (output) => { if output . index == this . next_outgoing_index { this . next_outgoing_index += 1 ; return Poll :: Ready (Some (output . data)) ; } else { this . queued_outputs . push (output) } } None => return Poll :: Ready (None) , } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
