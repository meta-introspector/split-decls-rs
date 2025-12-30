// Generated macro for from_public_bytes (function)
macro_rules! Depcrate_backend_x25519from_public_bytes {
() => {
// Module: crate::backend::x25519
// Provides: {"from_public_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_public_bytes (data : & [u8]) -> pyo3 :: PyResult < X25519PublicKey > { let pkey = openssl :: pkey :: PKey :: public_key_from_raw_bytes (data , openssl :: pkey :: Id :: X25519) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("An X25519 public key is 32 bytes long") }) ? ; Ok (X25519PublicKey { pkey }) }
};
}
