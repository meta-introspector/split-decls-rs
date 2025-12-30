// Generated macro for CryptographyError (enum)
macro_rules! Depcrate_errorCryptographyError {
() => {
// Module: crate::error
// Provides: {"CryptographyError"}
// Dependencies: {}
pub enum CryptographyError { Asn1Parse (asn1 :: ParseError) , Asn1Write (asn1 :: WriteError) , KeyParsing (asn1 :: ParseError) , Py (pyo3 :: PyErr) , OpenSSL (openssl :: error :: ErrorStack) , }
};
}
