// Generated macro for impl_43 (impl)
macro_rules! Depcrate_futureimpl_43 {
() => {
// Module: crate::future
// Provides: {"impl_43"}
// Dependencies: {}
# [cfg (feature = "race")] impl < T , F1 , F2 > Future for Race < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if this . rng . bool () { if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } } else { if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } } Poll :: Pending } }
};
}
