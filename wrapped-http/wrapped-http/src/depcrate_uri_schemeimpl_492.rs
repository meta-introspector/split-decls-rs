// Generated macro for impl_492 (impl)
macro_rules! Depcrate_uri_schemeimpl_492 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_492"}
// Dependencies: {}
# [doc = " Case-insensitive hashing"] impl Hash for Scheme { fn hash < H > (& self , state : & mut H) where H : Hasher , { match self . inner { Scheme2 :: None => () , Scheme2 :: Standard (Protocol :: Http) => state . write_u8 (1) , Scheme2 :: Standard (Protocol :: Https) => state . write_u8 (2) , Scheme2 :: Other (ref other) => { other . len () . hash (state) ; for & b in other . as_bytes () { state . write_u8 (b . to_ascii_lowercase ()) ; } } } } }
};
}
