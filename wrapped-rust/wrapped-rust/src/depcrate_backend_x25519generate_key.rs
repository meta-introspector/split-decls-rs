// Generated macro for generate_key (function)
macro_rules! Depcrate_backend_x25519generate_key {
() => {
// Module: crate::backend::x25519
// Provides: {"generate_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_key () -> CryptographyResult < X25519PrivateKey > { Ok (X25519PrivateKey { pkey : openssl :: pkey :: PKey :: generate_x25519 () ? , }) }
};
}
