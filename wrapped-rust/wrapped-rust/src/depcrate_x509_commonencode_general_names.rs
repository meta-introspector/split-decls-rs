// Generated macro for encode_general_names (function)
macro_rules! Depcrate_x509_commonencode_general_names {
() => {
// Module: crate::x509::common
// Provides: {"encode_general_names"}
// Dependencies: {}
pub (crate) fn encode_general_names < 'a > (py : pyo3 :: Python < '_ > , ka_bytes : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , ka_str : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedStr > , py_gns : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> Result < Vec < GeneralName < 'a > > , CryptographyError > { let mut gns = vec ! [] ; for el in py_gns . try_iter () ? { let gn = encode_general_name (py , ka_bytes , ka_str , & el ?) ? ; gns . push (gn) ; } Ok (gns) }
};
}
