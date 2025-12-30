// Generated macro for impl_170 (impl)
macro_rules! Depcrate_streamimpl_170 {
() => {
// Module: crate::stream
// Provides: {"impl_170"}
// Dependencies: {}
impl < S : Stream , U : Stream < Item = S :: Item > > Stream for Chain < S , U > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if ! this . first . done { let next = ready ! (this . first . as_mut () . poll_next (cx)) ; if let Some (next) = next { return Poll :: Ready (Some (next)) ; } } if ! this . second . done { let next = ready ! (this . second . as_mut () . poll_next (cx)) ; if let Some (next) = next { return Poll :: Ready (Some (next)) ; } } if this . first . done && this . second . done { Poll :: Ready (None) } else { Poll :: Pending } } }
};
}
