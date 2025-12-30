// Generated macro for SocksV5 (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5SocksV5 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5
// Provides: {"SocksV5"}
// Dependencies: {}
# [doc = " Tunnel Proxy via SOCKSv5"] # [doc = ""] # [doc = " This is a connector that can be used by the `legacy::Client`. It wraps"] # [doc = " another connector, and after getting an underlying connection, it established"] # [doc = " a TCP tunnel over it using SOCKSv5."] # [derive (Debug , Clone)] pub struct SocksV5 < C > { inner : C , config : SocksConfig , }
};
}
