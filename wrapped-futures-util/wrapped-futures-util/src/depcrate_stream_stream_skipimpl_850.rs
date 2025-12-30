// Generated macro for impl_850 (impl)
macro_rules! Depcrate_stream_stream_skipimpl_850 {
() => {
// Module: crate::stream::stream::skip
// Provides: {"impl_850"}
// Dependencies: {}
impl < St : Stream > Stream for Skip < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { let mut this = self . project () ; while * this . remaining > 0 { if ready ! (this . stream . as_mut () . poll_next (cx)) . is_some () { * this . remaining -= 1 ; } else { return Poll :: Ready (None) ; } } this . stream . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_sub (self . remaining) ; let upper = upper . map (| x | x . saturating_sub (self . remaining)) ; (lower , upper) } }
};
}
