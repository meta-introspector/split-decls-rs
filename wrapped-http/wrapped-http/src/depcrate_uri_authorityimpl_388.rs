// Generated macro for impl_388 (impl)
macro_rules! Depcrate_uri_authorityimpl_388 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_388"}
// Dependencies: {}
# [doc = " Case-insensitive equality"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Authority;"] # [doc = " let authority: Authority = \"HELLO.com\".parse().unwrap();"] # [doc = " assert_eq!(authority, \"hello.coM\");"] # [doc = " assert_eq!(\"hello.com\", authority);"] # [doc = " ```"] impl PartialEq < str > for Authority { fn eq (& self , other : & str) -> bool { self . data . eq_ignore_ascii_case (other) } }
};
}
