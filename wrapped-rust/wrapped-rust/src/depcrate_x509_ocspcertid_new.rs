// Generated macro for certid_new (function)
macro_rules! Depcrate_x509_ocspcertid_new {
() => {
// Module: crate::x509::ocsp
// Provides: {"certid_new"}
// Dependencies: {}
pub (crate) fn certid_new < 'p > (py : pyo3 :: Python < 'p > , ka : & 'p cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , cert : & 'p Certificate , issuer : & 'p Certificate , hash_algorithm : & pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < CertID < 'p > > { let issuer_der = asn1 :: write_single (& cert . raw . borrow_dependent () . tbs_cert . issuer) ? ; let issuer_name_hash = pyo3 :: pybacked :: PyBackedBytes :: from (hash_data (py , hash_algorithm , & issuer_der) ?) ; let issuer_key_hash = pyo3 :: pybacked :: PyBackedBytes :: from (hash_data (py , hash_algorithm , issuer . raw . borrow_dependent () . tbs_cert . spki . subject_public_key . as_bytes () ,) ?) ; Ok (CertID { hash_algorithm : HASH_NAME_TO_ALGORITHM_IDENTIFIERS [& * hash_algorithm . getattr (pyo3 :: intern ! (py , "name")) ? . extract :: < pyo3 :: pybacked :: PyBackedStr > () ?] . clone () , issuer_name_hash : ka . add (issuer_name_hash) , issuer_key_hash : ka . add (issuer_key_hash) , serial_number : cert . raw . borrow_dependent () . tbs_cert . serial , }) }
};
}
