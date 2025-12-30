// Generated macro for impl_895 (impl)
macro_rules! Depcrate_stream_stream_take_whileimpl_895 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"impl_895"}
// Dependencies: {}
impl < St , Fut , F > Stream for TakeWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { if self . done_taking { return Poll :: Ready (None) ; } let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let take = ready ! (fut . poll (cx)) ; let item = this . pending_item . take () ; this . pending_fut . set (None) ; if take { break item ; } else { * this . done_taking = true ; break None ; } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . pending_fut . set (Some ((this . f) (& item))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { if self . done_taking { return (0 , Some (0)) ; } let pending_len = usize :: from (self . pending_item . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } }
};
}
