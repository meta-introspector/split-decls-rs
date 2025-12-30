// Generated macro for impl_35 (impl)
macro_rules! Depcrate_combinators_collectimpl_35 {
() => {
// Module: crate::combinators::collect
// Provides: {"impl_35"}
// Dependencies: {}
impl < T : Body + ? Sized > Future for Collect < T > { type Output = Result < crate :: Collected < T :: Data > , T :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> std :: task :: Poll < Self :: Output > { let mut me = self . project () ; loop { let frame = ready ! (me . body . as_mut () . poll_frame (cx)) ; let frame = if let Some (frame) = frame { frame ? } else { return Poll :: Ready (Ok (me . collected . take () . expect ("polled after complete"))) ; } ; me . collected . as_mut () . unwrap () . push_frame (frame) ; } } }
};
}
