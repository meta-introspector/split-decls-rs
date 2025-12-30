// Generated macro for impl_612 (impl)
macro_rules! Depcrate_service_oneshotimpl_612 {
() => {
// Module: crate::service::oneshot
// Provides: {"impl_612"}
// Dependencies: {}
impl < S , Req > Future for Oneshot < S , Req > where S : Service < Req > , { type Output = Result < S :: Response , S :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let this = self . as_mut () . project () ; match this { OneshotProj :: NotReady { svc , req } => { ready ! (svc . poll_ready (cx)) ? ; let fut = svc . call (req . take () . expect ("already called")) ; self . set (Oneshot :: Called { fut }) ; } OneshotProj :: Called { fut } => { let res = ready ! (fut . poll (cx)) ? ; self . set (Oneshot :: Done) ; return Poll :: Ready (Ok (res)) ; } OneshotProj :: Done => panic ! ("polled after complete") , } } } }
};
}
