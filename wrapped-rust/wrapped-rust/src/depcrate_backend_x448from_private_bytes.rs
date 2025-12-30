// Generated macro for from_private_bytes (function)
macro_rules! Depcrate_backend_x448from_private_bytes {
() => {
// Module: crate::backend::x448
// Provides: {"from_private_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_private_bytes (data : CffiBuf < '_ >) -> pyo3 :: PyResult < X448PrivateKey > { let pkey = openssl :: pkey :: PKey :: private_key_from_raw_bytes (data . as_bytes () , openssl :: pkey :: Id :: X448) . map_err (| e | { pyo3 :: exceptions :: PyValueError :: new_err (format ! ("An X448 private key is 56 bytes long: {e}")) }) ? ; Ok (X448PrivateKey { pkey }) }
};
}
