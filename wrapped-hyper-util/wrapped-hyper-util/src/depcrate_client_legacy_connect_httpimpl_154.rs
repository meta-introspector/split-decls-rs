// Generated macro for impl_154 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_154 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a > ConnectingTcp < 'a > { fn new (remote_addrs : dns :: SocketAddrs , config : & 'a Config) -> Self { if let Some (fallback_timeout) = config . happy_eyeballs_timeout { let (preferred_addrs , fallback_addrs) = remote_addrs . split_by_preference (config . local_address_ipv4 , config . local_address_ipv6) ; if fallback_addrs . is_empty () { return ConnectingTcp { preferred : ConnectingTcpRemote :: new (preferred_addrs , config . connect_timeout) , fallback : None , config , } ; } ConnectingTcp { preferred : ConnectingTcpRemote :: new (preferred_addrs , config . connect_timeout) , fallback : Some (ConnectingTcpFallback { delay : tokio :: time :: sleep (fallback_timeout) , remote : ConnectingTcpRemote :: new (fallback_addrs , config . connect_timeout) , }) , config , } } else { ConnectingTcp { preferred : ConnectingTcpRemote :: new (remote_addrs , config . connect_timeout) , fallback : None , config , } } } }
};
}
