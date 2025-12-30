// Generated macro for impl_168 (impl)
macro_rules! Depcrate_streamimpl_168 {
() => {
// Module: crate::stream
// Provides: {"impl_168"}
// Dependencies: {}
impl < S : Stream > Stream for StepBy < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => { if * this . i == 0 { * this . i = * this . step - 1 ; return Poll :: Ready (Some (v)) ; } else { * this . i -= 1 ; } } None => return Poll :: Ready (None) , } } } }
};
}
