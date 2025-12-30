// Generated macro for hkdf_extract (function)
macro_rules! Depcrate_backend_kdfhkdf_extract {
() => {
// Module: crate::backend::kdf
// Provides: {"hkdf_extract"}
// Dependencies: {}
fn hkdf_extract (py : pyo3 :: Python < '_ > , algorithm : & pyo3 :: Py < pyo3 :: PyAny > , salt : Option < & pyo3 :: Py < pyo3 :: types :: PyBytes > > , key_material : & CffiBuf < '_ > ,) -> CryptographyResult < cryptography_openssl :: hmac :: DigestBytes > { let algorithm_bound = algorithm . bind (py) ; let digest_size = algorithm_bound . getattr (pyo3 :: intern ! (py , "digest_size")) ? . extract :: < usize > () ? ; let salt_bound = salt . map (| s | s . bind (py)) ; let default_salt = vec ! [0 ; digest_size] ; let salt_bytes : & [u8] = if let Some (bound) = salt_bound { bound . as_bytes () } else { & default_salt } ; let mut hmac = Hmac :: new_bytes (py , salt_bytes , algorithm_bound) ? ; hmac . update_bytes (key_material . as_bytes ()) ? ; hmac . finalize_bytes () }
};
}
