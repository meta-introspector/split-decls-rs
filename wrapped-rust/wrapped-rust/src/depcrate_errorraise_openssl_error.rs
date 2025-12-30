// Generated macro for raise_openssl_error (function)
macro_rules! Depcrate_errorraise_openssl_error {
() => {
// Module: crate::error
// Provides: {"raise_openssl_error"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn raise_openssl_error () -> crate :: error :: CryptographyResult < () > { Err (openssl :: error :: ErrorStack :: get () . into ()) }
};
}
