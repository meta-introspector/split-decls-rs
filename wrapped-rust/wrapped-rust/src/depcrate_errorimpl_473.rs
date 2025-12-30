// Generated macro for impl_473 (impl)
macro_rules! Depcrate_errorimpl_473 {
() => {
// Module: crate::error
// Provides: {"impl_473"}
// Dependencies: {}
impl From < CryptographyError > for pyo3 :: PyErr { fn from (e : CryptographyError) -> pyo3 :: PyErr { match e { CryptographyError :: Asn1Parse (_) | CryptographyError :: KeyParsing (_) => { pyo3 :: exceptions :: PyValueError :: new_err (e . to_string ()) } CryptographyError :: Asn1Write (asn1 :: WriteError :: AllocationError) => { pyo3 :: exceptions :: PyMemoryError :: new_err (e . to_string ()) } CryptographyError :: Py (py_error) => py_error , CryptographyError :: OpenSSL (ref error_stack) => pyo3 :: Python :: attach (| py | { let errors = list_from_openssl_error (py , error_stack) ; exceptions :: InternalError :: new_err ((e . to_string () , errors . unbind ())) }) , } } }
};
}
