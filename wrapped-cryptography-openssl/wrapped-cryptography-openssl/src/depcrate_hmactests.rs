// Generated macro for tests (module)
macro_rules! Depcrate_hmactests {
() => {
// Module: crate::hmac
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use openssl_sys as ffi ; use super :: DigestBytes ; # [test] fn test_digest_bytes () { let d = DigestBytes { buf : [19 ; ffi :: EVP_MAX_MD_SIZE as usize] , len : 12 , } ; assert_eq ! (&* d , b"\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13\x13") ; } }
};
}
