// Generated macro for impl_460 (impl)
macro_rules! Depcrate_uri_portimpl_460 {
() => {
// Module: crate::uri::port
// Provides: {"impl_460"}
// Dependencies: {}
impl < T > Port < T > { # [doc = " Returns the port number as a `u16`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Port as `u16`."] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Authority;"] # [doc = " let authority: Authority = \"example.org:80\".parse().unwrap();"] # [doc = ""] # [doc = " let port = authority.port().unwrap();"] # [doc = " assert_eq!(port.as_u16(), 80);"] # [doc = " ```"] pub const fn as_u16 (& self) -> u16 { self . port } }
};
}
