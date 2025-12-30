// Generated macro for handle_validation_error (function)
macro_rules! Depcrate_x509_verifyhandle_validation_error {
() => {
// Module: crate::x509::verify
// Provides: {"handle_validation_error"}
// Dependencies: {}
fn handle_validation_error < T > (py : pyo3 :: Python < '_ > , e : cryptography_x509_verification :: ValidationError < '_ , PyCryptoOps > ,) -> CryptographyResult < T > { let mut msg = format ! ("validation failed: {e}") ; if let Some (cert) = e . certificate () { let cert_repr = cert . extra () . bind (py) . repr () ? ; msg = format ! ("{msg} (encountered processing {cert_repr})") ; } Err (CryptographyError :: from (VerificationError :: new_err (msg))) }
};
}
