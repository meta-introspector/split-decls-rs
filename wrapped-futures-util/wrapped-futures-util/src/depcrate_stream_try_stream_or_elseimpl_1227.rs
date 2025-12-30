// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_stream_try_stream_or_elseimpl_1227 {
() => {
// Module: crate::stream::try_stream::or_else
// Provides: {"impl_1227"}
// Dependencies: {}
impl < St , Fut , F > Stream for OrElse < St , Fut , F > where St : TryStream , F : FnMut (St :: Error) -> Fut , Fut : TryFuture < Ok = St :: Ok > , { type Item = Result < St :: Ok , Fut :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let item = ready ! (fut . try_poll (cx)) ; this . future . set (None) ; break Some (item) ; } else { match ready ! (this . stream . as_mut () . try_poll_next (cx)) { Some (Ok (item)) => break Some (Ok (item)) , Some (Err (e)) => { this . future . set (Some ((this . f) (e))) ; } None => break None , } } }) } fn size_hint (& self) -> (usize , Option < usize >) { let future_len = usize :: from (self . future . is_some ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (future_len) ; let upper = match upper { Some (x) => x . checked_add (future_len) , None => None , } ; (lower , upper) } }
};
}
