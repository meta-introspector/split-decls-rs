// Generated macro for singleresp_py_next_update (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_next_update {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_next_update"}
// Dependencies: {}
fn singleresp_py_next_update < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { match & resp . next_update { Some (v) => x509 :: datetime_to_py (py , v . as_datetime ()) , None => Ok (py . None () . into_bound (py)) , } }
};
}
