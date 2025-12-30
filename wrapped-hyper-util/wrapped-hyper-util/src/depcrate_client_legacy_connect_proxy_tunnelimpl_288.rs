// Generated macro for impl_288 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelimpl_288 {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"impl_288"}
// Dependencies: {}
impl std :: fmt :: Display for TunnelError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("tunnel error: ") ? ; f . write_str (match self { TunnelError :: MissingHost => "missing destination host" , TunnelError :: ProxyAuthRequired => "proxy authorization required" , TunnelError :: ProxyHeadersTooLong => "proxy response headers too long" , TunnelError :: TunnelUnexpectedEof => "unexpected end of file" , TunnelError :: TunnelUnsuccessful => "unsuccessful" , TunnelError :: ConnectFailed (_) => "failed to create underlying connection" , TunnelError :: Io (_) => "io error establishing tunnel" , }) } }
};
}
