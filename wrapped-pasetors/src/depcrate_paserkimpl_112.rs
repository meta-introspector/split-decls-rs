// Generated macro for impl_112 (impl)
macro_rules! Depcrate_paserkimpl_112 {
() => {
// Module: crate::paserk
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "v3")] impl From < & AsymmetricSecretKey < V3 > > for Id { fn from (key : & AsymmetricSecretKey < V3 >) -> Self { let header = String :: from ("k3.sid.") ; let mut hasher = sha384 :: Sha384 :: new () ; hasher . update (header . as_bytes ()) . unwrap () ; let mut paserk_string = String :: new () ; key . fmt (& mut paserk_string) . unwrap () ; hasher . update (paserk_string . as_bytes ()) . unwrap () ; let identifier = encode_b64 (& hasher . finalize () . unwrap () . as_ref () [.. 33]) . unwrap () ; debug_assert_eq ! (identifier . len () , 44) ; Self { header , identifier } } }
};
}
