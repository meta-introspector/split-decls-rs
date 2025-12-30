// Generated macro for impl_98 (impl)
macro_rules! Depcrate_backend_cmacimpl_98 {
() => {
// Module: crate::backend::cmac
// Provides: {"impl_98"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Cmac { # [new] # [pyo3 (signature = (algorithm , backend = None))] fn new (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < Self > { let _ = backend ; Cmac :: new_with_algorithm (py , & algorithm) } fn update (& mut self , data : CffiBuf < '_ >) -> CryptographyResult < () > { self . update_bytes (data . as_bytes ()) } fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let data = self . finalize_bytes () ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & data)) } fn verify (& mut self , _py : pyo3 :: Python < '_ > , signature : & [u8]) -> CryptographyResult < () > { let actual = self . finalize_bytes () ? ; if ! constant_time :: bytes_eq (& actual , signature) { return Err (CryptographyError :: from (exceptions :: InvalidSignature :: new_err ("Signature did not match digest.") ,)) ; } Ok (()) } pub (crate) fn copy (& self) -> CryptographyResult < Cmac > { Ok (Cmac { ctx : Some (self . get_ctx () ? . copy () ?) , }) } }
};
}
