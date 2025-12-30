// Generated macro for datetime_to_py_utc_with_microseconds (function)
macro_rules! Depcrate_x509_commondatetime_to_py_utc_with_microseconds {
() => {
// Module: crate::x509::common
// Provides: {"datetime_to_py_utc_with_microseconds"}
// Dependencies: {}
pub (crate) fn datetime_to_py_utc_with_microseconds < 'p > (py : pyo3 :: Python < 'p > , dt : & asn1 :: DateTime , ms : u32 ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let year = dt . year () . into () ; let month = dt . month () ; let day = dt . day () ; let hour = dt . hour () ; let min = dt . minute () ; let sec = dt . second () ; let tzinfo = pyo3 :: types :: PyTzInfo :: utc (py) ? . to_owned () ; let py_datetime = pyo3 :: types :: PyDateTime :: new (py , year , month , day , hour , min , sec , ms , Some (& tzinfo)) ? ; Ok (py_datetime . into_any ()) }
};
}
