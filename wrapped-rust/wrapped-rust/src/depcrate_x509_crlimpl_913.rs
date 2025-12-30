// Generated macro for impl_913 (impl)
macro_rules! Depcrate_x509_crlimpl_913 {
() => {
// Module: crate::x509::crl
// Provides: {"impl_913"}
// Dependencies: {}
# [pyo3 :: pymethods] impl RevokedCertificate { # [getter] fn serial_number < 'p > (& self , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { big_byte_slice_to_py_int (py , self . owned . borrow_dependent () . user_certificate . as_bytes () ,) } # [getter] fn revocation_date < 'p > (& self , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let warning_cls = types :: DEPRECATED_IN_42 . get (py) ? ; let message = c"Properties that return a naïve datetime object have been deprecated. Please switch to revocation_date_utc." ; pyo3 :: PyErr :: warn (py , & warning_cls , message , 1) ? ; x509 :: datetime_to_py (py , self . owned . borrow_dependent () . revocation_date . as_datetime () ,) } # [getter] fn revocation_date_utc < 'p > (& self , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { x509 :: datetime_to_py_utc (py , self . owned . borrow_dependent () . revocation_date . as_datetime () ,) } # [getter] fn extensions (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < pyo3 :: Py < pyo3 :: PyAny > > { x509 :: parse_and_cache_extensions (py , & self . cached_extensions , & self . owned . borrow_dependent () . raw_crl_entry_extensions , | ext | parse_crl_entry_ext (py , ext) ,) } }
};
}
