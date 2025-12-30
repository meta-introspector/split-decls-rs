// Generated macro for impl_449 (impl)
macro_rules! Depcrate_declarative_asn1_typesimpl_449 {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"impl_449"}
// Dependencies: {}
# [pyo3 :: pymethods] impl UtcTime { # [new] # [pyo3 (signature = (inner ,))] fn new (py : pyo3 :: Python < '_ > , inner : pyo3 :: Py < pyo3 :: types :: PyDateTime >) -> pyo3 :: PyResult < Self > { if inner . bind (py) . get_tzinfo () . is_none () { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("invalid UtcTime: cannot initialize with naive datetime object" ,)) ; } let (datetime , microseconds) = crate :: x509 :: py_to_datetime_with_microseconds (py , inner . clone_ref (py) . into_bound (py)) ? ; if microseconds . is_some () { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("invalid UtcTime: fractional seconds are not supported" ,)) ; } Asn1UtcTime :: new (datetime) . map_err (| e | { pyo3 :: exceptions :: PyValueError :: new_err (format ! ("invalid UtcTime: {e}")) }) ? ; Ok (UtcTime { inner }) } # [pyo3 (signature = ())] pub fn as_datetime (& self , py : pyo3 :: Python < '_ > ,) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: types :: PyDateTime > > { Ok (self . inner . clone_ref (py)) } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self >) -> pyo3 :: PyResult < bool > { (* * self . inner . bind (py)) . eq (other . inner . bind (py)) } pub fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { Ok (format ! ("UtcTime({})" , self . inner . bind (py) . repr () ?)) } }
};
}
