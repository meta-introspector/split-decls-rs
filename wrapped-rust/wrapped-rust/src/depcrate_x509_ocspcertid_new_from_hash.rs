// Generated macro for certid_new_from_hash (function)
macro_rules! Depcrate_x509_ocspcertid_new_from_hash {
() => {
// Module: crate::x509::ocsp
// Provides: {"certid_new_from_hash"}
// Dependencies: {}
pub (crate) fn certid_new_from_hash < 'p > (py : pyo3 :: Python < 'p > , issuer_name_hash : & 'p [u8] , issuer_key_hash : & 'p [u8] , serial_number : asn1 :: BigInt < 'p > , hash_algorithm : pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> CryptographyResult < CertID < 'p > > { let hash_name = hash_algorithm . getattr (pyo3 :: intern ! (py , "name")) ? . extract :: < pyo3 :: pybacked :: PyBackedStr > () ? ; Ok (CertID { hash_algorithm : HASH_NAME_TO_ALGORITHM_IDENTIFIERS [& * hash_name] . clone () , issuer_name_hash , issuer_key_hash , serial_number , }) }
};
}
