// Generated macro for generate_key (function)
macro_rules! Depcrate_backend_x448generate_key {
() => {
// Module: crate::backend::x448
// Provides: {"generate_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_key () -> CryptographyResult < X448PrivateKey > { Ok (X448PrivateKey { pkey : openssl :: pkey :: PKey :: generate_x448 () ? , }) }
};
}
