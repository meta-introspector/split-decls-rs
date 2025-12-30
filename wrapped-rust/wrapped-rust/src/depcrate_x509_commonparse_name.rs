// Generated macro for parse_name (function)
macro_rules! Depcrate_x509_commonparse_name {
() => {
// Module: crate::x509::common
// Provides: {"parse_name"}
// Dependencies: {}
pub (crate) fn parse_name < 'p > (py : pyo3 :: Python < 'p > , name : & NameReadable < '_ > ,) -> Result < pyo3 :: Bound < 'p , pyo3 :: PyAny > , CryptographyError > { let py_rdns = pyo3 :: types :: PyList :: empty (py) ; for rdn in name . clone () { let py_rdn = parse_rdn (py , & rdn) ? ; py_rdns . append (py_rdn) ? ; } Ok (types :: NAME . get (py) ? . call1 ((py_rdns ,)) ?) }
};
}
