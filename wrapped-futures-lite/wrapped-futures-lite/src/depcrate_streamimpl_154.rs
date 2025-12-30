// Generated macro for impl_154 (impl)
macro_rules! Depcrate_streamimpl_154 {
() => {
// Module: crate::stream
// Provides: {"impl_154"}
// Dependencies: {}
# [cfg (feature = "race")] impl < T , S1 , S2 > Stream for Race < S1 , S2 > where S1 : Stream < Item = T > , S2 : Stream < Item = T > , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if this . rng . bool () { if let Poll :: Ready (Some (t)) = this . stream1 . as_mut () . poll_next (cx) { return Poll :: Ready (Some (t)) ; } if let Poll :: Ready (Some (t)) = this . stream2 . as_mut () . poll_next (cx) { return Poll :: Ready (Some (t)) ; } } else { if let Poll :: Ready (Some (t)) = this . stream2 . as_mut () . poll_next (cx) { return Poll :: Ready (Some (t)) ; } if let Poll :: Ready (Some (t)) = this . stream1 . as_mut () . poll_next (cx) { return Poll :: Ready (Some (t)) ; } } Poll :: Pending } }
};
}
