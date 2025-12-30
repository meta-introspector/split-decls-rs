// Generated macro for from_private_bytes (function)
macro_rules! Depcrate_backend_x25519from_private_bytes {
() => {
// Module: crate::backend::x25519
// Provides: {"from_private_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_private_bytes (data : CffiBuf < '_ >) -> pyo3 :: PyResult < X25519PrivateKey > { let pkey = openssl :: pkey :: PKey :: private_key_from_raw_bytes (data . as_bytes () , openssl :: pkey :: Id :: X25519) . map_err (| e | { pyo3 :: exceptions :: PyValueError :: new_err (format ! ("An X25519 private key is 32 bytes long: {e}")) }) ? ; Ok (X25519PrivateKey { pkey }) }
};
}
