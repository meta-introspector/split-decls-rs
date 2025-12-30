// Generated macro for accept (function)
macro_rules! Depcrateaccept {
() => {
// Module: crate
// Provides: {"accept"}
// Dependencies: {}
# [doc = " Creates a new server-side connection."] # [doc = ""] # [doc = " The `scid` parameter represents the server's source connection ID, while"] # [doc = " the optional `odcid` parameter represents the original destination ID the"] # [doc = " client sent before a stateless retry (this is only required when using"] # [doc = " the [`retry()`] function)."] # [doc = ""] # [doc = " [`retry()`]: fn.retry.html"] # [doc = ""] # [doc = " ## Examples:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let mut config = quiche::Config::new(0xbabababa)?;"] # [doc = " # let scid = quiche::ConnectionId::from_ref(&[0xba; 16]);"] # [doc = " # let local = \"127.0.0.1:0\".parse().unwrap();"] # [doc = " # let peer = \"127.0.0.1:1234\".parse().unwrap();"] # [doc = " let conn = quiche::accept(&scid, None, local, peer, &mut config)?;"] # [doc = " # Ok::<(), quiche::Error>(())"] # [doc = " ```"] # [inline] pub fn accept (scid : & ConnectionId , odcid : Option < & ConnectionId > , local : SocketAddr , peer : SocketAddr , config : & mut Config ,) -> Result < Connection > { let conn = Connection :: new (scid , odcid , local , peer , config , true) ? ; Ok (conn) }
};
}
