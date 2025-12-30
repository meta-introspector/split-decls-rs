// Generated macro for generate_key (function)
macro_rules! Depcrate_backend_ed25519generate_key {
() => {
// Module: crate::backend::ed25519
// Provides: {"generate_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_key () -> CryptographyResult < Ed25519PrivateKey > { Ok (Ed25519PrivateKey { pkey : openssl :: pkey :: PKey :: generate_ed25519 () ? , }) }
};
}
