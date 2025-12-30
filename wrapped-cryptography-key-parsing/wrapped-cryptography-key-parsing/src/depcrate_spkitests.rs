// Generated macro for tests (module)
macro_rules! Depcrate_spkitests {
() => {
// Module: crate::spki
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: serialize_public_key ; # [cfg (not (CRYPTOGRAPHY_IS_BORINGSSL))] # [test] # [should_panic (expected = "Unknown key type")] fn test_serialize_public_key_unknown_key_type () { let pkey = openssl :: pkey :: PKey :: hmac (& [0u8 ; 16]) . unwrap () ; _ = serialize_public_key (& pkey) ; } # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC , CRYPTOGRAPHY_IS_LIBRESSL)))] # [test] # [should_panic (expected = "Unknown curve")] fn test_serialize_public_key_unknown_curve () { let pkey = openssl :: pkey :: PKey :: ec_gen ("brainpoolP512t1") . unwrap () ; _ = serialize_public_key (& pkey) ; } }
};
}
