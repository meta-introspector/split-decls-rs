// Generated macro for parse_policy_qualifiers (function)
macro_rules! Depcrate_x509_certificateparse_policy_qualifiers {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_policy_qualifiers"}
// Dependencies: {}
fn parse_policy_qualifiers < 'a > (py : pyo3 :: Python < 'a > , policy_qualifiers : & asn1 :: SequenceOf < 'a , PolicyQualifierInfo < 'a , Asn1Read > > ,) -> CryptographyResult < pyo3 :: Bound < 'a , pyo3 :: PyAny > > { let py_pq = pyo3 :: types :: PyList :: empty (py) ; for pqi in policy_qualifiers . clone () { let qualifier = match pqi . qualifier { Qualifier :: CpsUri (data) => { if pqi . policy_qualifier_id == oid :: CP_CPS_URI_OID { pyo3 :: types :: PyString :: new (py , data . as_str ()) . into_any () } else { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("CpsUri ASN.1 structure found but OID did not match" ,) ,)) ; } } Qualifier :: UserNotice (un) => { if pqi . policy_qualifier_id != oid :: CP_USER_NOTICE_OID { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("UserNotice ASN.1 structure found but OID did not match" ,) ,)) ; } parse_user_notice (py , un) ? } } ; py_pq . append (qualifier) ? ; } Ok (py_pq . into_any ()) }
};
}
