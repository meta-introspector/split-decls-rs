// Generated macro for impl_119 (impl)
macro_rules! Depcrate_paserkimpl_119 {
() => {
// Module: crate::paserk
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (feature = "v4")] impl From < & AsymmetricPublicKey < V4 > > for Id { fn from (key : & AsymmetricPublicKey < V4 >) -> Self { let header = String :: from ("k4.pid.") ; let mut hasher = blake2b :: Blake2b :: new (33) . unwrap () ; hasher . update (header . as_bytes ()) . unwrap () ; let mut paserk_string = String :: new () ; key . fmt (& mut paserk_string) . unwrap () ; hasher . update (paserk_string . as_bytes ()) . unwrap () ; let identifier = encode_b64 (hasher . finalize () . unwrap () . as_ref ()) . unwrap () ; debug_assert_eq ! (identifier . len () , 44) ; Self { header , identifier } } }
};
}
