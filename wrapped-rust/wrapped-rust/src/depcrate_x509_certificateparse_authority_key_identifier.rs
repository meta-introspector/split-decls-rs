// Generated macro for parse_authority_key_identifier (function)
macro_rules! Depcrate_x509_certificateparse_authority_key_identifier {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_authority_key_identifier"}
// Dependencies: {}
pub (crate) fn parse_authority_key_identifier < 'p > (py : pyo3 :: Python < 'p > , ext : & Extension < 'p > ,) -> Result < pyo3 :: Bound < 'p , pyo3 :: PyAny > , CryptographyError > { let aki = ext . value :: < AuthorityKeyIdentifier < '_ , Asn1Read > > () ? ; let serial = match aki . authority_cert_serial_number { Some (biguint) => { warn_if_not_positive (py , biguint . as_bytes ()) ? ; big_byte_slice_to_py_int (py , biguint . as_bytes ()) ? . unbind () } None => py . None () , } ; let issuer = match aki . authority_cert_issuer { Some (aci) => x509 :: parse_general_names (py , & aci) ? , None => py . None () . into_bound (py) , } ; Ok (types :: AUTHORITY_KEY_IDENTIFIER . get (py) ? . call1 ((aki . key_identifier , issuer , serial)) ?) }
};
}
