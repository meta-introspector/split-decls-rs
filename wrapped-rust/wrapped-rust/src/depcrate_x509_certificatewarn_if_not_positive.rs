// Generated macro for warn_if_not_positive (function)
macro_rules! Depcrate_x509_certificatewarn_if_not_positive {
() => {
// Module: crate::x509::certificate
// Provides: {"warn_if_not_positive"}
// Dependencies: {}
fn warn_if_not_positive (py : pyo3 :: Python < '_ > , bytes : & [u8]) -> pyo3 :: PyResult < () > { if bytes [0] & 0x80 != 0 || bytes == [0] { let warning_cls = types :: DEPRECATED_IN_36 . get (py) ? ; let message = c"Parsed a serial number which wasn't positive (i.e., it was negative or zero), which is disallowed by RFC 5280. Loading this certificate will cause an exception in a future release of cryptography." ; pyo3 :: PyErr :: warn (py , & warning_cls , message , 1) ? ; } Ok (()) }
};
}
