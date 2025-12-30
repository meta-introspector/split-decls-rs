// Generated macro for singleresp_py_revocation_time_utc (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_revocation_time_utc {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_revocation_time_utc"}
// Dependencies: {}
fn singleresp_py_revocation_time_utc < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { match & resp . cert_status { ocsp_resp :: CertStatus :: Revoked (revoked_info) => { x509 :: datetime_to_py_utc (py , revoked_info . revocation_time . as_datetime ()) } ocsp_resp :: CertStatus :: Good (_) | ocsp_resp :: CertStatus :: Unknown (_) => { Ok (py . None () . into_bound (py)) } } }
};
}
