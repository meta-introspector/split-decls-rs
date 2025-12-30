// Generated macro for impl_45 (impl)
macro_rules! Depcrate_systemimpl_45 {
() => {
// Module: crate::system
// Provides: {"impl_45"}
// Dependencies: {}
impl Future for SystemController { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . cmd_rx . poll_recv (cx)) { None => return Poll :: Ready (()) , Some (cmd) => match cmd { SystemCommand :: Exit (code) => { for arb in self . arbiters . values () { arb . stop () ; } if let Some (stop_tx) = self . stop_tx . take () { let _ = stop_tx . send (code) ; } } SystemCommand :: RegisterArbiter (id , arb) => { self . arbiters . insert (id , arb) ; } SystemCommand :: DeregisterArbiter (id) => { self . arbiters . remove (& id) ; } } , } } } }
};
}
