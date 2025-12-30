// Generated macro for impl_866 (impl)
macro_rules! Depcrate_stream_stream_skip_whileimpl_866 {
() => {
// Module: crate::stream::stream::skip_while
// Provides: {"impl_866"}
// Dependencies: {}
impl < St , Fut , F > Stream for SkipWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { let mut this = self . project () ; if * this . done_skipping { return this . stream . poll_next (cx) ; } Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let skipped = ready ! (fut . poll (cx)) ; let item = this . pending_item . take () ; this . pending_fut . set (None) ; if ! skipped { * this . done_skipping = true ; break item ; } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . pending_fut . set (Some ((this . f) (& item))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { if self . done_skipping { self . stream . size_hint () } else { let pending_len = usize :: from (self . pending_item . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } } }
};
}
