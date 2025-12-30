// Generated macro for parse_rdn (function)
macro_rules! Depcrate_x509_commonparse_rdn {
() => {
// Module: crate::x509::common
// Provides: {"parse_rdn"}
// Dependencies: {}
pub (crate) fn parse_rdn < 'a > (py : pyo3 :: Python < 'a > , rdn : & asn1 :: SetOf < 'a , AttributeTypeValue < 'a > > ,) -> CryptographyResult < pyo3 :: Bound < 'a , pyo3 :: PyAny > > { let py_attrs = pyo3 :: types :: PyList :: empty (py) ; for attribute in rdn . clone () { let na = parse_name_attribute (py , attribute) ? ; py_attrs . append (na) ? ; } Ok (types :: RELATIVE_DISTINGUISHED_NAME . get (py) ? . call1 ((py_attrs ,)) ?) }
};
}
