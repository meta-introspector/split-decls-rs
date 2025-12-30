// Generated macro for from_public_bytes (function)
macro_rules! Depcrate_backend_ed25519from_public_bytes {
() => {
// Module: crate::backend::ed25519
// Provides: {"from_public_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_public_bytes (data : & [u8]) -> pyo3 :: PyResult < Ed25519PublicKey > { let pkey = openssl :: pkey :: PKey :: public_key_from_raw_bytes (data , openssl :: pkey :: Id :: ED25519) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("An Ed25519 public key is 32 bytes long") }) ? ; Ok (Ed25519PublicKey { pkey }) }
};
}
