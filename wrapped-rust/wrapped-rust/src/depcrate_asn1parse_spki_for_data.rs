// Generated macro for parse_spki_for_data (function)
macro_rules! Depcrate_asn1parse_spki_for_data {
() => {
// Module: crate::asn1
// Provides: {"parse_spki_for_data"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn parse_spki_for_data < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] ,) -> Result < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > , CryptographyError > { let spki = asn1 :: parse_single :: < SubjectPublicKeyInfo < '_ > > (data) ? ; if spki . subject_public_key . padding_bits () != 0 { return Err (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid public key encoding") . into ()) ; } Ok (pyo3 :: types :: PyBytes :: new (py , spki . subject_public_key . as_bytes () ,)) }
};
}
