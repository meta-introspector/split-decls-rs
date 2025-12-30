// Generated macro for impl_534 (impl)
macro_rules! Depcrate_utilsimpl_534 {
() => {
// Module: crate::utils
// Provides: {"impl_534"}
// Dependencies: {}
impl < A > ActorFuture < A > for TimerFunc < A > where A : Actor , { type Output = () ; fn poll (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { let this = self . project () ; ready ! (this . timeout . poll (task)) ; let f = this . f . take () . expect ("TimerFunc polled after finish") ; f (act , ctx) ; Poll :: Ready (()) } }
};
}
