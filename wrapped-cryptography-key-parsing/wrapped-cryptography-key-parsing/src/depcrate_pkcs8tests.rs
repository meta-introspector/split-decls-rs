// Generated macro for tests (module)
macro_rules! Depcrate_pkcs8tests {
() => {
// Module: crate::pkcs8
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: serialize_private_key ; # [cfg (not (CRYPTOGRAPHY_IS_BORINGSSL))] # [test] # [should_panic (expected = "Unknown key type")] fn test_serialize_private_key_unknown_key_type () { let pkey = openssl :: pkey :: PKey :: hmac (& [0u8 ; 16]) . unwrap () ; _ = serialize_private_key (& pkey) ; } }
};
}
