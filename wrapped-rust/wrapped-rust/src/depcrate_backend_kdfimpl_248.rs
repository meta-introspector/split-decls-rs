// Generated macro for impl_248 (impl)
macro_rules! Depcrate_backend_kdfimpl_248 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_248"}
// Dependencies: {}
impl Pbkdf2Hmac { fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } openssl :: pkcs5 :: pbkdf2_hmac (key_material , self . salt . as_bytes (py) , self . iterations , self . md , output ,) . unwrap () ; Ok (self . length) } }
};
}
