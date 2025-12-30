// Generated macro for BoxHandshaking (type)
macro_rules! Depcrate_client_legacy_connect_proxy_socksBoxHandshaking {
() => {
// Module: crate::client::legacy::connect::proxy::socks
// Provides: {"BoxHandshaking"}
// Dependencies: {}
type BoxHandshaking < T , E > = Pin < Box < dyn Future < Output = Result < T , SocksError < E > > > + Send > > ;
};
}
