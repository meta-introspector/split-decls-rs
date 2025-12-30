// Generated macro for GaiFuture (struct)
macro_rules! Depcrate_client_legacy_connect_dnsGaiFuture {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"GaiFuture"}
// Dependencies: {}
# [doc = " A future to resolve a name returned by `GaiResolver`."] pub struct GaiFuture { inner : JoinHandle < Result < SocketAddrs , io :: Error > > , }
};
}
