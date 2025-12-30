// Generated macro for check_key_infinity (function)
macro_rules! Depcrate_backend_eccheck_key_infinity {
() => {
// Module: crate::backend::ec
// Provides: {"check_key_infinity"}
// Dependencies: {}
fn check_key_infinity (ec : & openssl :: ec :: EcKeyRef < impl openssl :: pkey :: HasPublic > ,) -> CryptographyResult < () > { if ec . public_key () . is_infinity (ec . group ()) { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Cannot load an EC public key where the point is at infinity" ,) ,)) ; } Ok (()) }
};
}
