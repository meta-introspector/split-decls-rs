// Generated macro for impl_461 (impl)
macro_rules! Depcrate_uri_portimpl_461 {
() => {
// Module: crate::uri::port
// Provides: {"impl_461"}
// Dependencies: {}
impl < T > Port < T > where T : AsRef < str > , { # [doc = " Converts a `str` to a port number."] # [doc = ""] # [doc = " The supplied `str` must be a valid u16."] pub (crate) fn from_str (bytes : T) -> Result < Self , InvalidUri > { bytes . as_ref () . parse :: < u16 > () . map (| port | Port { port , repr : bytes }) . map_err (| _ | ErrorKind :: InvalidPort . into ()) } # [doc = " Returns the port number as a `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Port as `str`."] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Authority;"] # [doc = " let authority: Authority = \"example.org:80\".parse().unwrap();"] # [doc = ""] # [doc = " let port = authority.port().unwrap();"] # [doc = " assert_eq!(port.as_str(), \"80\");"] # [doc = " ```"] pub fn as_str (& self) -> & str { self . repr . as_ref () } }
};
}
