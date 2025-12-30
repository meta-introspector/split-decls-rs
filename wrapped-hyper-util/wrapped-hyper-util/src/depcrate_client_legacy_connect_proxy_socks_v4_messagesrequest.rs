// Generated macro for Request (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_messagesRequest {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::messages
// Provides: {"Request"}
// Dependencies: {}
# [doc = " +-----+-----+----+----+----+----+----+----+-------------+------+------------+------+"] # [doc = " |  VN |  CD | DSTPORT |        DSTIP      |    USERID   | NULL |   DOMAIN   | NULL |"] # [doc = " +-----+-----+----+----+----+----+----+----+-------------+------+------------+------+"] # [doc = " |  1  |  1  |    2    |         4         |   Variable  |  1   |  Variable  |   1  |"] # [doc = " +-----+-----+----+----+----+----+----+----+-------------+------+------------+------+"] # [doc = "                                                                ^^^^^^^^^^^^^^^^^^^^^"] # [doc = "                                                      optional: only do IP is 0.0.0.X"] # [derive (Debug)] pub struct Request < 'a > (pub & 'a Address) ;
};
}
