// Generated macro for list_from_openssl_error (function)
macro_rules! Depcrate_errorlist_from_openssl_error {
() => {
// Module: crate::error
// Provides: {"list_from_openssl_error"}
// Dependencies: {}
pub (crate) fn list_from_openssl_error < 'p > (py : pyo3 :: Python < 'p > , error_stack : & openssl :: error :: ErrorStack ,) -> pyo3 :: Bound < 'p , pyo3 :: types :: PyList > { let errors = pyo3 :: types :: PyList :: empty (py) ; for e in error_stack . errors () { errors . append (pyo3 :: Bound :: new (py , OpenSSLError { e : e . clone () }) . expect ("Failed to create OpenSSLError") ,) . expect ("Failed to append to list") ; } errors }
};
}
