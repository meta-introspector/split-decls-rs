// Generated macro for oaep_hash_supported (function)
macro_rules! Depcrate_backend_rsaoaep_hash_supported {
() => {
// Module: crate::backend::rsa
// Provides: {"oaep_hash_supported"}
// Dependencies: {}
fn oaep_hash_supported (md : & openssl :: hash :: MessageDigest) -> bool { md == & openssl :: hash :: MessageDigest :: sha1 () || md == & openssl :: hash :: MessageDigest :: sha224 () || md == & openssl :: hash :: MessageDigest :: sha256 () || md == & openssl :: hash :: MessageDigest :: sha384 () || md == & openssl :: hash :: MessageDigest :: sha512 () }
};
}
