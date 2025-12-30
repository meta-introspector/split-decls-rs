// Generated macro for parse_general_names (function)
macro_rules! Depcrate_x509_commonparse_general_names {
() => {
// Module: crate::x509::common
// Provides: {"parse_general_names"}
// Dependencies: {}
pub (crate) fn parse_general_names < 'a > (py : pyo3 :: Python < 'a > , gn_seq : & asn1 :: SequenceOf < 'a , GeneralName < 'a > > ,) -> CryptographyResult < pyo3 :: Bound < 'a , pyo3 :: PyAny > > { let gns = pyo3 :: types :: PyList :: empty (py) ; for gn in gn_seq . clone () { let py_gn = parse_general_name (py , gn) ? ; gns . append (py_gn) ? ; } Ok (gns . into_any ()) }
};
}
