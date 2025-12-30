// Generated macro for TunnelError (enum)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelTunnelError {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"TunnelError"}
// Dependencies: {}
# [derive (Debug)] pub enum TunnelError { ConnectFailed (Box < dyn StdError + Send + Sync >) , Io (std :: io :: Error) , MissingHost , ProxyAuthRequired , ProxyHeadersTooLong , TunnelUnexpectedEof , TunnelUnsuccessful , }
};
}
