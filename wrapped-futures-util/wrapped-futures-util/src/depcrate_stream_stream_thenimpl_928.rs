// Generated macro for impl_928 (impl)
macro_rules! Depcrate_stream_stream_thenimpl_928 {
() => {
// Module: crate::stream::stream::then
// Provides: {"impl_928"}
// Dependencies: {}
impl < St , Fut , F > Stream for Then < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future , { type Item = Fut :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let item = ready ! (fut . poll (cx)) ; this . future . set (None) ; break Some (item) ; } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . future . set (Some ((this . f) (item))) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { let future_len = usize :: from (self . future . is_some ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (future_len) ; let upper = match upper { Some (x) => x . checked_add (future_len) , None => None , } ; (lower , upper) } }
};
}
