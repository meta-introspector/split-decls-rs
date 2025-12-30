// Generated macro for impl_1406 (impl)
macro_rules! Depcrate_stream_try_stream_try_unfoldimpl_1406 {
() => {
// Module: crate::stream::try_stream::try_unfold
// Provides: {"impl_1406"}
// Dependencies: {}
impl < T , F , Fut , Item > Stream for TryUnfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : TryFuture < Ok = Option < (Item , T) > > , { type Item = Result < Item , Fut :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Some (state) = this . state . take () { this . fut . set (Some ((this . f) (state))) ; } match this . fut . as_mut () . as_pin_mut () { None => { Poll :: Ready (None) } Some (future) => { let step = ready ! (future . try_poll (cx)) ; this . fut . set (None) ; match step { Ok (Some ((item , next_state))) => { * this . state = Some (next_state) ; Poll :: Ready (Some (Ok (item))) } Ok (None) => Poll :: Ready (None) , Err (e) => Poll :: Ready (Some (Err (e))) , } } } } }
};
}
