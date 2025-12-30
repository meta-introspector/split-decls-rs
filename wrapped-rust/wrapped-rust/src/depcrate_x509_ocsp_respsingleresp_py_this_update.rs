// Generated macro for singleresp_py_this_update (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_this_update {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_this_update"}
// Dependencies: {}
fn singleresp_py_this_update < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { x509 :: datetime_to_py (py , resp . this_update . as_datetime ()) }
};
}
