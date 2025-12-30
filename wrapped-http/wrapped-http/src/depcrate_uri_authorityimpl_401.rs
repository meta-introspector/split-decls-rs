// Generated macro for impl_401 (impl)
macro_rules! Depcrate_uri_authorityimpl_401 {
() => {
// Module: crate::uri::authority
// Provides: {"impl_401"}
// Dependencies: {}
# [doc = " Case-insensitive hashing"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::uri::Authority;"] # [doc = " # use std::hash::{Hash, Hasher};"] # [doc = " # use std::collections::hash_map::DefaultHasher;"] # [doc = ""] # [doc = " let a: Authority = \"HELLO.com\".parse().unwrap();"] # [doc = " let b: Authority = \"hello.coM\".parse().unwrap();"] # [doc = ""] # [doc = " let mut s = DefaultHasher::new();"] # [doc = " a.hash(&mut s);"] # [doc = " let a = s.finish();"] # [doc = ""] # [doc = " let mut s = DefaultHasher::new();"] # [doc = " b.hash(&mut s);"] # [doc = " let b = s.finish();"] # [doc = ""] # [doc = " assert_eq!(a, b);"] # [doc = " ```"] impl Hash for Authority { fn hash < H > (& self , state : & mut H) where H : Hasher , { self . data . len () . hash (state) ; for & b in self . data . as_bytes () { state . write_u8 (b . to_ascii_lowercase ()) ; } } }
};
}
