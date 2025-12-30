// Generated macro for from_private_bytes (function)
macro_rules! Depcrate_backend_ed25519from_private_bytes {
() => {
// Module: crate::backend::ed25519
// Provides: {"from_private_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_private_bytes (data : CffiBuf < '_ >) -> pyo3 :: PyResult < Ed25519PrivateKey > { let pkey = openssl :: pkey :: PKey :: private_key_from_raw_bytes (data . as_bytes () , openssl :: pkey :: Id :: ED25519 ,) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("An Ed25519 private key is 32 bytes long") }) ? ; Ok (Ed25519PrivateKey { pkey }) }
};
}
