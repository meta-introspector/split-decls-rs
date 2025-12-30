// Generated macro for from_public_bytes (function)
macro_rules! Depcrate_backend_x448from_public_bytes {
() => {
// Module: crate::backend::x448
// Provides: {"from_public_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_public_bytes (data : & [u8]) -> pyo3 :: PyResult < X448PublicKey > { let pkey = openssl :: pkey :: PKey :: public_key_from_raw_bytes (data , openssl :: pkey :: Id :: X448) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("An X448 public key is 32 bytes long") }) ? ; Ok (X448PublicKey { pkey }) }
};
}
