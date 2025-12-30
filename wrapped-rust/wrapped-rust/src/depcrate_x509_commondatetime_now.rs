// Generated macro for datetime_now (function)
macro_rules! Depcrate_x509_commondatetime_now {
() => {
// Module: crate::x509::common
// Provides: {"datetime_now"}
// Dependencies: {}
pub (crate) fn datetime_now (py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < asn1 :: DateTime > { let utc = pyo3 :: types :: PyTzInfo :: utc (py) ? ; py_to_datetime (py , types :: DATETIME_DATETIME . get (py) ? . call_method1 (pyo3 :: intern ! (py , "now") , (utc ,)) ? . extract () ? ,) }
};
}
