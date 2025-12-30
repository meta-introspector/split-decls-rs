// Generated macro for py_to_datetime (function)
macro_rules! Depcrate_x509_commonpy_to_datetime {
() => {
// Module: crate::x509::common
// Provides: {"py_to_datetime"}
// Dependencies: {}
pub (crate) fn py_to_datetime (py : pyo3 :: Python < '_ > , val : pyo3 :: Bound < '_ , pyo3 :: types :: PyDateTime > ,) -> pyo3 :: PyResult < asn1 :: DateTime > { let (datetime , _) = py_to_datetime_with_microseconds (py , val) ? ; Ok (datetime) }
};
}
