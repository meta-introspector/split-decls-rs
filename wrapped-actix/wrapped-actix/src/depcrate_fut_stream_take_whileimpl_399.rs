// Generated macro for impl_399 (impl)
macro_rules! Depcrate_fut_stream_take_whileimpl_399 {
() => {
// Module: crate::fut::stream::take_while
// Provides: {"impl_399"}
// Dependencies: {}
impl < S , A , F , Fut > ActorStream < A > for TakeWhile < S , S :: Item , F , Fut > where S : ActorStream < A > , A : Actor , F : FnMut (& S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A , Output = bool > , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ > ,) -> Poll < Option < Self :: Item > > { if self . done_taking { return Poll :: Ready (None) ; } let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let take = ready ! (fut . poll (act , ctx , task)) ; let item = this . pending_item . take () ; this . pending_fut . set (None) ; if take { break item ; } else { * this . done_taking = true ; break None ; } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) { this . pending_fut . set (Some ((this . f) (& item , act , ctx))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } }
};
}
