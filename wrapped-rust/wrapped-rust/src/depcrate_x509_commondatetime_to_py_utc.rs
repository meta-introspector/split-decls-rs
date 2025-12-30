// Generated macro for datetime_to_py_utc (function)
macro_rules! Depcrate_x509_commondatetime_to_py_utc {
() => {
// Module: crate::x509::common
// Provides: {"datetime_to_py_utc"}
// Dependencies: {}
pub (crate) fn datetime_to_py_utc < 'p > (py : pyo3 :: Python < 'p > , dt : & asn1 :: DateTime ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { datetime_to_py_utc_with_microseconds (py , dt , 0) }
};
}
