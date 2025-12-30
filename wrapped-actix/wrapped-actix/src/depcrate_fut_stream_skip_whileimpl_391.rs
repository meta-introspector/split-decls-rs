// Generated macro for impl_391 (impl)
macro_rules! Depcrate_fut_stream_skip_whileimpl_391 {
() => {
// Module: crate::fut::stream::skip_while
// Provides: {"impl_391"}
// Dependencies: {}
impl < S , A , F , Fut > ActorStream < A > for SkipWhile < S , S :: Item , F , Fut > where S : ActorStream < A > , A : Actor , F : FnMut (& S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A , Output = bool > , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ > ,) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if * this . done_skipping { return this . stream . poll_next (act , ctx , task) ; } Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let skipped = ready ! (fut . poll (act , ctx , task)) ; let item = this . pending_item . take () ; this . pending_fut . set (None) ; if ! skipped { * this . done_skipping = true ; break item ; } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) { this . pending_fut . set (Some ((this . f) (& item , act , ctx))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } }
};
}
