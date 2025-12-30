// Generated macro for impl_192 (impl)
macro_rules! Depcrate_backend_ed25519impl_192 {
() => {
// Module: crate::backend::ed25519
// Provides: {"impl_192"}
// Dependencies: {}
# [pyo3 :: pymethods] impl Ed25519PublicKey { fn verify (& self , signature : CffiBuf < '_ > , data : CffiBuf < '_ >) -> CryptographyResult < () > { let valid = openssl :: sign :: Verifier :: new_without_digest (& self . pkey) ? . verify_oneshot (signature . as_bytes () , data . as_bytes ()) . unwrap_or (false) ; if ! valid { return Err (CryptographyError :: from (exceptions :: InvalidSignature :: new_err (()) ,)) ; } Ok (()) } fn public_bytes_raw < 'p > (& self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let raw_bytes = self . pkey . raw_public_key () ? ; Ok (pyo3 :: types :: PyBytes :: new (py , & raw_bytes)) } fn public_bytes < 'p > (slf : & pyo3 :: Bound < 'p , Self > , py : pyo3 :: Python < 'p > , encoding : & pyo3 :: Bound < 'p , pyo3 :: PyAny > , format : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { utils :: pkey_public_bytes (py , slf , & slf . borrow () . pkey , encoding , format , true , true) } fn __eq__ (& self , other : pyo3 :: PyRef < '_ , Self >) -> bool { self . pkey . public_eq (& other . pkey) } fn __copy__ (slf : pyo3 :: PyRef < '_ , Self >) -> pyo3 :: PyRef < '_ , Self > { slf } fn __deepcopy__ < 'p > (slf : pyo3 :: PyRef < 'p , Self > , _memo : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> pyo3 :: PyRef < 'p , Self > { slf } }
};
}
