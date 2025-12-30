// Generated macro for verify_signature_with_signature_algorithm (function)
macro_rules! Depcrate_x509_signverify_signature_with_signature_algorithm {
() => {
// Module: crate::x509::sign
// Provides: {"verify_signature_with_signature_algorithm"}
// Dependencies: {}
pub (crate) fn verify_signature_with_signature_algorithm < 'p > (py : pyo3 :: Python < 'p > , issuer_public_key : pyo3 :: Bound < 'p , pyo3 :: PyAny > , signature_algorithm : & common :: AlgorithmIdentifier < '_ > , signature : & [u8] , data : & [u8] ,) -> CryptographyResult < () > { let key_type = identify_public_key_type (py , issuer_public_key . clone ()) ? ; let sig_key_type = identify_key_type_for_algorithm_params (& signature_algorithm . params) ? ; if key_type != sig_key_type { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Signature algorithm does not match issuer key type" ,) ,)) ; } let py_signature_algorithm_parameters = identify_signature_algorithm_parameters (py , signature_algorithm) ? ; let py_signature_hash_algorithm = identify_signature_hash_algorithm (py , signature_algorithm) ? ; match key_type { KeyType :: Ed25519 | KeyType :: Ed448 => { issuer_public_key . call_method1 (pyo3 :: intern ! (py , "verify") , (signature , data)) ? } KeyType :: Ec => issuer_public_key . call_method1 (pyo3 :: intern ! (py , "verify") , (signature , data , py_signature_algorithm_parameters) ,) ? , KeyType :: Rsa => issuer_public_key . call_method1 (pyo3 :: intern ! (py , "verify") , (signature , data , py_signature_algorithm_parameters , py_signature_hash_algorithm ,) ,) ? , KeyType :: Dsa => issuer_public_key . call_method1 (pyo3 :: intern ! (py , "verify") , (signature , data , py_signature_hash_algorithm) ,) ? , } ; Ok (()) }
};
}
