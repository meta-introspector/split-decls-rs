// Generated macro for impl_481 (impl)
macro_rules! Depcrate_uri_schemeimpl_481 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_481"}
// Dependencies: {}
impl Scheme { # [doc = " HTTP protocol scheme"] pub const HTTP : Scheme = Scheme { inner : Scheme2 :: Standard (Protocol :: Http) , } ; # [doc = " HTTP protocol over TLS."] pub const HTTPS : Scheme = Scheme { inner : Scheme2 :: Standard (Protocol :: Https) , } ; pub (super) fn empty () -> Self { Scheme { inner : Scheme2 :: None , } } # [doc = " Return a str representation of the scheme"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::*;"] # [doc = " let scheme: Scheme = \"http\".parse().unwrap();"] # [doc = " assert_eq!(scheme.as_str(), \"http\");"] # [doc = " ```"] # [inline] pub fn as_str (& self) -> & str { use self :: Protocol :: * ; use self :: Scheme2 :: * ; match self . inner { Standard (Http) => "http" , Standard (Https) => "https" , Other (ref v) => & v [..] , None => unreachable ! () , } } }
};
}
