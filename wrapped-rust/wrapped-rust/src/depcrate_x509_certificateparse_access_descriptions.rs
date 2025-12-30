// Generated macro for parse_access_descriptions (function)
macro_rules! Depcrate_x509_certificateparse_access_descriptions {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_access_descriptions"}
// Dependencies: {}
pub (crate) fn parse_access_descriptions < 'p > (py : pyo3 :: Python < 'p > , ext : & Extension < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let ads = pyo3 :: types :: PyList :: empty (py) ; let parsed = ext . value :: < SequenceOfAccessDescriptions < '_ , Asn1Read > > () ? ; for access in parsed { let py_oid = oid_to_py_oid (py , & access . access_method) ? ; let gn = x509 :: parse_general_name (py , access . access_location) ? ; let ad = types :: ACCESS_DESCRIPTION . get (py) ? . call1 ((py_oid , gn)) ? ; ads . append (ad) ? ; } Ok (ads . into_any ()) }
};
}
