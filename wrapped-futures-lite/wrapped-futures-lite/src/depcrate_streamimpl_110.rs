// Generated macro for impl_110 (impl)
macro_rules! Depcrate_streamimpl_110 {
() => {
// Module: crate::stream
// Provides: {"impl_110"}
// Dependencies: {}
impl < St , Fut > Stream for StopAfterFuture < St , Fut > where St : Stream , Fut : Future , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { let mut this = self . project () ; if let Some (f) = this . fut . as_mut () . as_pin_mut () { if let Poll :: Ready (result) = f . poll (cx) { this . fut . set (None) ; * this . fut_result = Some (result) ; } } if ! * this . free && this . fut . is_none () { Poll :: Ready (None) } else { let item = ready ! (this . stream . poll_next (cx)) ; if item . is_none () { this . fut . set (None) ; } Poll :: Ready (item) } } fn size_hint (& self) -> (usize , Option < usize >) { if self . is_stopped () { return (0 , Some (0)) ; } let (_ , upper_bound) = self . stream . size_hint () ; (0 , upper_bound) } }
};
}
