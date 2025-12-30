// Generated macro for connect (function)
macro_rules! Depcrateconnect {
() => {
// Module: crate
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Creates a new client-side connection."] # [doc = ""] # [doc = " The `scid` parameter is used as the connection's source connection ID,"] # [doc = " while the optional `server_name` parameter is used to verify the peer's"] # [doc = " certificate."] # [doc = ""] # [doc = " ## Examples:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let mut config = quiche::Config::new(0xbabababa)?;"] # [doc = " # let server_name = \"quic.tech\";"] # [doc = " # let scid = quiche::ConnectionId::from_ref(&[0xba; 16]);"] # [doc = " # let local = \"127.0.0.1:4321\".parse().unwrap();"] # [doc = " # let peer = \"127.0.0.1:1234\".parse().unwrap();"] # [doc = " let conn ="] # [doc = "     quiche::connect(Some(&server_name), &scid, local, peer, &mut config)?;"] # [doc = " # Ok::<(), quiche::Error>(())"] # [doc = " ```"] # [inline] pub fn connect (server_name : Option < & str > , scid : & ConnectionId , local : SocketAddr , peer : SocketAddr , config : & mut Config ,) -> Result < Connection > { let mut conn = Connection :: new (scid , None , local , peer , config , false) ? ; if let Some (server_name) = server_name { conn . handshake . set_host_name (server_name) ? ; } Ok (conn) }
};
}
