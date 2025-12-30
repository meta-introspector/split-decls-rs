// Generated macro for ProxyReq (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesProxyReq {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"ProxyReq"}
// Dependencies: {}
# [doc = " +----+-----+-------+------+----------+----------+"] # [doc = " |VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |"] # [doc = " +----+-----+-------+------+----------+----------+"] # [doc = " | 1  |  1  | X'00' |  1   | Variable |    2     |"] # [doc = " +----+-----+-------+------+----------+----------+"] # [derive (Debug)] pub struct ProxyReq < 'a > (pub & 'a Address) ;
};
}
