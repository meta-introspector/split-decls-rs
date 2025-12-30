// Generated macro for impl_244 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4impl_244 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4
// Provides: {"impl_244"}
// Dependencies: {}
impl < C > SocksV4 < C > { # [doc = " Create a new SOCKSv4 handshake service"] # [doc = ""] # [doc = " Wraps an underlying connector and stores the address of a tunneling"] # [doc = " proxying server."] # [doc = ""] # [doc = " A `SocksV4` can then be called with any destination. The `dst` passed to"] # [doc = " `call` will not be used to create the underlying connection, but will"] # [doc = " be used in a SOCKS handshake with the proxy destination."] pub fn new (proxy_dst : Uri , connector : C) -> Self { Self { inner : connector , config : SocksConfig :: new (proxy_dst) , } } # [doc = " Resolve domain names locally on the client, rather than on the proxy server."] # [doc = ""] # [doc = " Disabled by default as local resolution of domain names can be detected as a"] # [doc = " DNS leak."] pub fn local_dns (mut self , local_dns : bool) -> Self { self . config . local_dns = local_dns ; self } }
};
}
