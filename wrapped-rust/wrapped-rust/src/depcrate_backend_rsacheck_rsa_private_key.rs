// Generated macro for check_rsa_private_key (function)
macro_rules! Depcrate_backend_rsacheck_rsa_private_key {
() => {
// Module: crate::backend::rsa
// Provides: {"check_rsa_private_key"}
// Dependencies: {}
fn check_rsa_private_key (rsa : & openssl :: rsa :: Rsa < openssl :: pkey :: Private > ,) -> CryptographyResult < () > { if ! rsa . check_key () . unwrap_or (false) || rsa . p () . unwrap () . is_even () || rsa . q () . unwrap () . is_even () { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid private key") ,)) ; } Ok (()) }
};
}
