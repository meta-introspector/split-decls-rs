// Generated macro for datetime_to_py (function)
macro_rules! Depcrate_x509_commondatetime_to_py {
() => {
// Module: crate::x509::common
// Provides: {"datetime_to_py"}
// Dependencies: {}
pub (crate) fn datetime_to_py < 'p > (py : pyo3 :: Python < 'p > , dt : & asn1 :: DateTime ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let py_datetime = pyo3 :: types :: PyDateTime :: new (py , dt . year () . into () , dt . month () , dt . day () , dt . hour () , dt . minute () , dt . second () , 0 , None ,) ? ; Ok (py_datetime . into_any ()) }
};
}
