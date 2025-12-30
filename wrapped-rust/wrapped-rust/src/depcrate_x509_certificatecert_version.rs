// Generated macro for cert_version (function)
macro_rules! Depcrate_x509_certificatecert_version {
() => {
// Module: crate::x509::certificate
// Provides: {"cert_version"}
// Dependencies: {}
fn cert_version (py : pyo3 :: Python < '_ > , version : u8 ,) -> Result < pyo3 :: Bound < '_ , pyo3 :: PyAny > , CryptographyError > { match version { 0 => Ok (types :: CERTIFICATE_VERSION_V1 . get (py) ?) , 2 => Ok (types :: CERTIFICATE_VERSION_V3 . get (py) ?) , _ => Err (CryptographyError :: from (exceptions :: InvalidVersion :: new_err ((format ! ("{version} is not a valid X509 version") , version ,)) ,)) , } }
};
}
