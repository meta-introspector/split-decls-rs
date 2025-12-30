// Generated macro for impl_19 (impl)
macro_rules! Depcrate_arbiterimpl_19 {
() => {
// Module: crate::arbiter
// Provides: {"impl_19"}
// Dependencies: {}
impl Future for ArbiterRunner { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . rx . poll_recv (cx)) { None => return Poll :: Ready (()) , Some (item) => match item { ArbiterCommand :: Stop => { return Poll :: Ready (()) ; } ArbiterCommand :: Execute (task_fut) => { tokio :: task :: spawn_local (task_fut) ; } } , } } } }
};
}
