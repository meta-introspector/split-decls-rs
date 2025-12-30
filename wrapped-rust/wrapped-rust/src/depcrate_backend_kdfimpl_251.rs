// Generated macro for impl_251 (impl)
macro_rules! Depcrate_backend_kdfimpl_251 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_251"}
// Dependencies: {}
impl Scrypt { # [cfg (not (CRYPTOGRAPHY_IS_LIBRESSL))] fn derive_into_buffer (& mut self , py : pyo3 :: Python < '_ > , key_material : & [u8] , output : & mut [u8] ,) -> CryptographyResult < usize > { if self . used { return Err (exceptions :: already_finalized_error ()) ; } self . used = true ; if output . len () != self . length { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("buffer must be {} bytes" , self . length)) ,)) ; } openssl :: pkcs5 :: scrypt (key_material , self . salt . as_bytes (py) , self . n , self . r , self . p , (usize :: MAX / 2) . try_into () . unwrap () , output ,) . map_err (| _ | { let min_memory = 128 * self . n * self . r / (1024 * 1024) ; CryptographyError :: from (pyo3 :: exceptions :: PyMemoryError :: new_err (format ! ("Not enough memory to derive key. These parameters require {min_memory}MB of memory."))) }) ? ; Ok (self . length) } }
};
}
