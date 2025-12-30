// Generated macro for time_from_py (function)
macro_rules! Depcrate_x509_certificatetime_from_py {
() => {
// Module: crate::x509::certificate
// Provides: {"time_from_py"}
// Dependencies: {}
pub (crate) fn time_from_py (py : pyo3 :: Python < '_ > , val : & pyo3 :: Bound < '_ , pyo3 :: types :: PyDateTime > ,) -> CryptographyResult < common :: Time > { let dt = x509 :: py_to_datetime (py , val . clone ()) ? ; time_from_datetime (dt) }
};
}
