// Generated macro for impl_490 (impl)
macro_rules! Depcrate_uri_schemeimpl_490 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_490"}
// Dependencies: {}
# [doc = " Case-insensitive equality"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Scheme;"] # [doc = " let scheme: Scheme = \"HTTP\".parse().unwrap();"] # [doc = " assert_eq!(scheme, *\"http\");"] # [doc = " ```"] impl PartialEq < str > for Scheme { fn eq (& self , other : & str) -> bool { self . as_str () . eq_ignore_ascii_case (other) } }
};
}
