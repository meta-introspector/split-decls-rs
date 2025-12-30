// Generated macro for impl_35 (impl)
macro_rules! Depcrate_futureimpl_35 {
() => {
// Module: crate::future
// Provides: {"impl_35"}
// Dependencies: {}
impl < T , F1 , F2 > Future for Or < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } Poll :: Pending } }
};
}
