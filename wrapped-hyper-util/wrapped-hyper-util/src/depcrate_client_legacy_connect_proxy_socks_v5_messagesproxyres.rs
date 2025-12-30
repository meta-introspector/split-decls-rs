// Generated macro for ProxyRes (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesProxyRes {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"ProxyRes"}
// Dependencies: {}
# [doc = " +----+-----+-------+------+----------+----------+"] # [doc = " |VER | REP |  RSV  | ATYP | BND.ADDR | BND.PORT |"] # [doc = " +----+-----+-------+------+----------+----------+"] # [doc = " | 1  |  1  | X'00' |  1   | Variable |    2     |"] # [doc = " +----+-----+-------+------+----------+----------+"] # [derive (Debug)] pub struct ProxyRes (pub Status) ;
};
}
