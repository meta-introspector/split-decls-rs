// Generated macro for SocksV4 (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4SocksV4 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4
// Provides: {"SocksV4"}
// Dependencies: {}
# [doc = " Tunnel Proxy via SOCKSv4"] # [doc = ""] # [doc = " This is a connector that can be used by the `legacy::Client`. It wraps"] # [doc = " another connector, and after getting an underlying connection, it established"] # [doc = " a TCP tunnel over it using SOCKSv4."] # [derive (Debug , Clone)] pub struct SocksV4 < C > { inner : C , config : SocksConfig , }
};
}
