// Generated macro for impl_233 (impl)
macro_rules! Depcrate_backend_hmacimpl_233 {
() => {
// Module: crate::backend::hmac
// Provides: {"impl_233"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Hmac { # [new] # [pyo3 (signature = (key , algorithm , backend = None))] fn new (py : pyo3 :: Python < '_ > , key : CffiBuf < '_ > , algorithm : & pyo3 :: Bound < '_ , pyo3 :: PyAny > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < Hmac > { let _ = backend ; Hmac :: new_bytes (py , key . as_bytes () , algorithm) } fn update (& mut self , data : CffiBuf < '_ >) -> CryptographyResult < () > { self . update_bytes (data . as_bytes ()) } pub (crate) fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let data = self . finalize_bytes () ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & data)) } fn verify (& mut self , signature : & [u8]) -> CryptographyResult < () > { let actual = self . finalize_bytes () ? ; if ! constant_time :: bytes_eq (& actual , signature) { return Err (CryptographyError :: from (exceptions :: InvalidSignature :: new_err ("Signature did not match digest.") ,)) ; } Ok (()) } pub (crate) fn copy (& self , py : pyo3 :: Python < '_ >) -> CryptographyResult < Hmac > { Ok (Hmac { ctx : Some (self . get_ctx () ? . copy () ?) , algorithm : self . algorithm . clone_ref (py) , }) } }
};
}
