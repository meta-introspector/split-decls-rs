// Generated macro for AuthenticationReq (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_messagesAuthenticationReq {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::messages
// Provides: {"AuthenticationReq"}
// Dependencies: {}
# [doc = " +----+------+----------+------+----------+"] # [doc = " |VER | ULEN |  UNAME   | PLEN |  PASSWD  |"] # [doc = " +----+------+----------+------+----------+"] # [doc = " | 1  |  1   | 1 to 255 |  1   | 1 to 255 |"] # [doc = " +----+------+----------+------+----------+"] # [derive (Debug)] pub struct AuthenticationReq < 'a > (pub & 'a str , pub & 'a str) ;
};
}
