// Generated macro for generate_key (function)
macro_rules! Depcrate_backend_ed448generate_key {
() => {
// Module: crate::backend::ed448
// Provides: {"generate_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_key () -> CryptographyResult < Ed448PrivateKey > { Ok (Ed448PrivateKey { pkey : openssl :: pkey :: PKey :: generate_ed448 () ? , }) }
};
}
