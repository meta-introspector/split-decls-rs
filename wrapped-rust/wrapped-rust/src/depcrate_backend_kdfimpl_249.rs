// Generated macro for impl_249 (impl)
macro_rules! Depcrate_backend_kdfimpl_249 {
() => {
// Module: crate::backend::kdf
// Provides: {"impl_249"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Pbkdf2Hmac { # [new] # [pyo3 (signature = (algorithm , length , salt , iterations , backend = None))] fn new (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , length : usize , salt : pyo3 :: Py < pyo3 :: types :: PyBytes > , iterations : usize , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < Self > { _ = backend ; let md = hashes :: message_digest_from_algorithm (py , & algorithm) ? ; Ok (Pbkdf2Hmac { md , salt , iterations , length , used : false , }) } fn derive_into (& mut self , py : pyo3 :: Python < '_ > , key_material : CffiBuf < '_ > , mut buf : CffiMutBuf < '_ > ,) -> CryptographyResult < usize > { self . derive_into_buffer (py , key_material . as_bytes () , buf . as_mut_bytes ()) } fn derive < 'p > (& mut self , py : pyo3 :: Python < 'p > , key_material : CffiBuf < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { Ok (pyo3 :: types :: PyBytes :: new_with (py , self . length , | output | { self . derive_into_buffer (py , key_material . as_bytes () , output) ? ; Ok (()) }) ?) } fn verify (& mut self , py : pyo3 :: Python < '_ > , key_material : CffiBuf < '_ > , expected_key : CffiBuf < '_ > ,) -> CryptographyResult < () > { let actual = self . derive (py , key_material) ? ; let actual_bytes = actual . as_bytes () ; let expected_bytes = expected_key . as_bytes () ; if ! constant_time :: bytes_eq (actual_bytes , expected_bytes) { return Err (CryptographyError :: from (exceptions :: InvalidKey :: new_err ("Keys do not match." ,))) ; } Ok (()) } }
};
}
