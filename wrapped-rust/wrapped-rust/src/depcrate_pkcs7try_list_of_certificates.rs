// Generated macro for try_list_of_certificates (function)
macro_rules! Depcrate_pkcs7try_list_of_certificates {
() => {
// Module: crate::pkcs7
// Provides: {"try_list_of_certificates"}
// Dependencies: {}
pub fn try_list_of_certificates < 'p , F > (py : pyo3 :: Python < 'p > , data : pyo3 :: Py < pyo3 :: types :: PyBytes > , f : F ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyList > > where F : for < 'a > FnOnce (& 'a pyo3 :: Py < pyo3 :: types :: PyBytes > , & mut dyn FnMut (RawCertificate < 'a >) -> CryptographyResult < () > ,) -> CryptographyResult < () > , { let result = pyo3 :: types :: PyList :: empty (py) ; let mut cb = | val | { let raw_cert = certificate :: OwnedCertificate :: new (data . clone_ref (py) , | _ | unsafe { mem :: transmute (val) }) ; result . append (pyo3 :: Bound :: new (py , x509 :: certificate :: Certificate { raw : raw_cert , cached_extensions : pyo3 :: sync :: PyOnceLock :: new () , } ,) ?) ? ; Ok (()) } ; f (& data , & mut cb) ? ; Ok (result) }
};
}
