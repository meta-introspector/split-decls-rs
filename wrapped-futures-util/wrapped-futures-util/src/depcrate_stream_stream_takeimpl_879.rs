// Generated macro for impl_879 (impl)
macro_rules! Depcrate_stream_stream_takeimpl_879 {
() => {
// Module: crate::stream::stream::take
// Provides: {"impl_879"}
// Dependencies: {}
impl < St > Stream for Take < St > where St : Stream , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { if self . remaining == 0 { Poll :: Ready (None) } else { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; if next . is_some () { * this . remaining -= 1 ; } else { * this . remaining = 0 ; } Poll :: Ready (next) } } fn size_hint (& self) -> (usize , Option < usize >) { if self . remaining == 0 { return (0 , Some (0)) ; } let (lower , upper) = self . stream . size_hint () ; let lower = cmp :: min (lower , self . remaining) ; let upper = match upper { Some (x) if x < self . remaining => Some (x) , _ => Some (self . remaining) , } ; (lower , upper) } }
};
}
