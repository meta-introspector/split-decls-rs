// Generated macro for parse_crl_entry_ext (function)
macro_rules! Depcrate_x509_crlparse_crl_entry_ext {
() => {
// Module: crate::x509::crl
// Provides: {"parse_crl_entry_ext"}
// Dependencies: {}
pub fn parse_crl_entry_ext < 'p > (py : pyo3 :: Python < 'p > , ext : & Extension < 'p > ,) -> CryptographyResult < Option < pyo3 :: Bound < 'p , pyo3 :: PyAny > > > { match ext . extn_id { oid :: CRL_REASON_OID => { let flags = parse_crl_reason_flags (py , & ext . value :: < crl :: CRLReason > () ?) ? ; Ok (Some (types :: CRL_REASON . get (py) ? . call1 ((flags ,)) ?)) } oid :: CERTIFICATE_ISSUER_OID => { let gn_seq = ext . value :: < asn1 :: SequenceOf < '_ , name :: GeneralName < '_ > > > () ? ; let gns = x509 :: parse_general_names (py , & gn_seq) ? ; Ok (Some (types :: CERTIFICATE_ISSUER . get (py) ? . call1 ((gns ,)) ?)) } oid :: INVALIDITY_DATE_OID => { let time = ext . value :: < asn1 :: GeneralizedTime > () ? ; let py_dt = x509 :: datetime_to_py (py , time . as_datetime ()) ? ; Ok (Some (types :: INVALIDITY_DATE . get (py) ? . call1 ((py_dt ,)) ?)) } _ => Ok (None) , } }
};
}
