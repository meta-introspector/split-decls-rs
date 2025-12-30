// Generated macro for singleresp_py_certificate_status (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_certificate_status {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_certificate_status"}
// Dependencies: {}
fn singleresp_py_certificate_status < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let attr = match resp . cert_status { ocsp_resp :: CertStatus :: Good (_) => pyo3 :: intern ! (py , "GOOD") , ocsp_resp :: CertStatus :: Revoked (_) => pyo3 :: intern ! (py , "REVOKED") , ocsp_resp :: CertStatus :: Unknown (_) => pyo3 :: intern ! (py , "UNKNOWN") , } ; types :: OCSP_CERT_STATUS . get (py) ? . getattr (attr) }
};
}
