// Generated macro for singleresp_py_serial_number (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_serial_number {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_serial_number"}
// Dependencies: {}
fn singleresp_py_serial_number < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { big_byte_slice_to_py_int (py , resp . cert_id . serial_number . as_bytes ()) }
};
}
