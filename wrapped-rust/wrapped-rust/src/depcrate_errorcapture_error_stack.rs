// Generated macro for capture_error_stack (function)
macro_rules! Depcrate_errorcapture_error_stack {
() => {
// Module: crate::error
// Provides: {"capture_error_stack"}
// Dependencies: {}
# [pyo3 :: pyfunction] pub (crate) fn capture_error_stack (py : pyo3 :: Python < '_ > ,) -> pyo3 :: PyResult < pyo3 :: Bound < '_ , pyo3 :: types :: PyList > > { let errs = pyo3 :: types :: PyList :: empty (py) ; for e in openssl :: error :: ErrorStack :: get () . errors () { errs . append (pyo3 :: Bound :: new (py , OpenSSLError { e : e . clone () }) ?) ? ; } Ok (errs) }
};
}
