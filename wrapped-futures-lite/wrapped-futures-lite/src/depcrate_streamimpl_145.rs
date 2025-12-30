// Generated macro for impl_145 (impl)
macro_rules! Depcrate_streamimpl_145 {
() => {
// Module: crate::stream
// Provides: {"impl_145"}
// Dependencies: {}
impl < S , F , Fut > Stream for Then < S , F , Fut > where S : Stream , F : FnMut (S :: Item) -> Fut , Fut : Future , { type Item = Fut :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let item = ready ! (fut . poll (cx)) ; this . future . set (None) ; return Poll :: Ready (Some (item)) ; } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . future . set (Some ((this . f) (item))) ; } else { return Poll :: Ready (None) ; } } } fn size_hint (& self) -> (usize , Option < usize >) { let future_len = self . future . is_some () as usize ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (future_len) ; let upper = upper . and_then (| u | u . checked_add (future_len)) ; (lower , upper) } }
};
}
