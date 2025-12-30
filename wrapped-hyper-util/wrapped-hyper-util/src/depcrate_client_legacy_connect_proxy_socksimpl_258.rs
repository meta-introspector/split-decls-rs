// Generated macro for impl_258 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socksimpl_258 {
() => {
// Module: crate::client::legacy::connect::proxy::socks
// Provides: {"impl_258"}
// Dependencies: {}
impl < C > std :: fmt :: Display for SocksError < C > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("SOCKS error: ") ? ; match self { Self :: Inner (_) => f . write_str ("failed to create underlying connection") , Self :: Io (_) => f . write_str ("io error during SOCKS handshake") , Self :: DnsFailure => f . write_str ("could not resolve to acceptable address type") , Self :: MissingHost => f . write_str ("missing destination host") , Self :: MissingPort => f . write_str ("missing destination port") , Self :: Parsing (_) => f . write_str ("failed parsing server response") , Self :: Serialize (_) => f . write_str ("failed serialize request") , Self :: V4 (e) => e . fmt (f) , Self :: V5 (e) => e . fmt (f) , } } }
};
}
