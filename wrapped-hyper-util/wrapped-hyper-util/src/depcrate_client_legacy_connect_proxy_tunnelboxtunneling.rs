// Generated macro for BoxTunneling (type)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelBoxTunneling {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"BoxTunneling"}
// Dependencies: {}
type BoxTunneling < T > = Pin < Box < dyn Future < Output = Result < T , TunnelError > > + Send > > ;
};
}
