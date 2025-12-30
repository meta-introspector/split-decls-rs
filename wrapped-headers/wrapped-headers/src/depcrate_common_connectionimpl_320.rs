// Generated macro for impl_320 (impl)
macro_rules! Depcrate_common_connectionimpl_320 {
() => {
// Module: crate::common::connection
// Provides: {"impl_320"}
// Dependencies: {}
impl Connection { # [doc = " A constructor to easily create a `Connection: close` header."] # [inline] pub fn close () -> Connection { Connection (HeaderValue :: from_static ("close") . into ()) } # [doc = " A constructor to easily create a `Connection: keep-alive` header."] # [inline] pub fn keep_alive () -> Connection { Connection (HeaderValue :: from_static ("keep-alive") . into ()) } # [doc = " A constructor to easily create a `Connection: Upgrade` header."] # [inline] pub fn upgrade () -> Connection { Connection (HeaderValue :: from_static ("upgrade") . into ()) } # [doc = " Check if this header contains a given \"connection option\"."] # [doc = ""] # [doc = " This can be used with various argument types:"] # [doc = ""] # [doc = " - `&str`"] # [doc = " - `&HeaderName`"] # [doc = " - `HeaderName`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = ""] # [doc = " use http::header::UPGRADE;"] # [doc = " use headers::Connection;"] # [doc = ""] # [doc = " let conn = Connection::keep_alive();"] # [doc = ""] # [doc = " assert!(!conn.contains(\"close\"));"] # [doc = " assert!(!conn.contains(UPGRADE));"] # [doc = " assert!(conn.contains(\"keep-alive\"));"] # [doc = " assert!(conn.contains(\"Keep-Alive\"));"] # [doc = " ```"] pub fn contains (& self , name : impl AsConnectionOption) -> bool { let s = name . as_connection_option () ; self . 0 . iter () . any (| opt | opt . eq_ignore_ascii_case (s)) } }
};
}
