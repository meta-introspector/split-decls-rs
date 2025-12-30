// Generated macro for impl_375 (impl)
macro_rules! Depcrate_fut_stream_foldimpl_375 {
() => {
// Module: crate::fut::stream::fold
// Provides: {"impl_375"}
// Dependencies: {}
impl < S , A , F , Fut > ActorFuture < A > for Fold < S , F , Fut , Fut :: Output > where S : ActorStream < A > , A : Actor , F : FnMut (Fut :: Output , S :: Item , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A > , { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { * this . accum = Some (ready ! (fut . poll (act , ctx , task))) ; this . future . set (None) ; } else if this . accum . is_some () { let res = ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) ; let a = this . accum . take () . unwrap () ; if let Some (item) = res { this . future . set (Some ((this . f) (a , item , act , ctx))) ; } else { break a ; } } else { panic ! ("Fold polled after completion") } }) } }
};
}
