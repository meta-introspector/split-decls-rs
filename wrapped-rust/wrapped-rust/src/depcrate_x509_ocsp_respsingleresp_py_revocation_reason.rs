// Generated macro for singleresp_py_revocation_reason (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_revocation_reason {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_revocation_reason"}
// Dependencies: {}
fn singleresp_py_revocation_reason < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { match & resp . cert_status { ocsp_resp :: CertStatus :: Revoked (revoked_info) => match revoked_info . revocation_reason { Some (ref v) => Ok (crl :: parse_crl_reason_flags (py , v) ?) , None => Ok (py . None () . into_bound (py)) , } , ocsp_resp :: CertStatus :: Good (_) | ocsp_resp :: CertStatus :: Unknown (_) => { Ok (py . None () . into_bound (py)) } } }
};
}
