// Generated macro for tests (module)
macro_rules! Depcrate_backend_keystests {
() => {
// Module: crate::backend::keys
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] use super :: { private_key_from_pkey , public_key_from_pkey } ; # [test] # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] fn test_public_key_from_pkey_unknown_key () { pyo3 :: Python :: initialize () ; pyo3 :: Python :: attach (| py | { let pkey = openssl :: pkey :: PKey :: public_key_from_raw_bytes (& [0 ; 32] , openssl :: pkey :: Id :: X25519) . unwrap () ; assert ! (public_key_from_pkey (py , & pkey , openssl :: pkey :: Id :: CMAC) . is_err ()) ; }) ; } # [test] # [cfg (not (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] fn test_private_key_from_pkey_unknown_key () { pyo3 :: Python :: initialize () ; pyo3 :: Python :: attach (| py | { let pkey = openssl :: pkey :: PKey :: hmac (& [0 ; 32]) . unwrap () ; assert ! (private_key_from_pkey (py , & pkey , false) . is_err ()) ; }) ; } }
};
}
