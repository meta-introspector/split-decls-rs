// Generated macro for from_private_bytes (function)
macro_rules! Depcrate_backend_ed448from_private_bytes {
() => {
// Module: crate::backend::ed448
// Provides: {"from_private_bytes"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn from_private_bytes (data : CffiBuf < '_ >) -> pyo3 :: PyResult < Ed448PrivateKey > { let pkey = openssl :: pkey :: PKey :: private_key_from_raw_bytes (data . as_bytes () , openssl :: pkey :: Id :: ED448) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("An Ed448 private key is 57 bytes long") }) ? ; Ok (Ed448PrivateKey { pkey }) }
};
}
