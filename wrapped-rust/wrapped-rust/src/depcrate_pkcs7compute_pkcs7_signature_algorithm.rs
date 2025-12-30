// Generated macro for compute_pkcs7_signature_algorithm (function)
macro_rules! Depcrate_pkcs7compute_pkcs7_signature_algorithm {
() => {
// Module: crate::pkcs7
// Provides: {"compute_pkcs7_signature_algorithm"}
// Dependencies: {}
fn compute_pkcs7_signature_algorithm < 'p > (py : pyo3 :: Python < 'p > , private_key : pyo3 :: Bound < 'p , pyo3 :: PyAny > , hash_algorithm : pyo3 :: Bound < 'p , pyo3 :: PyAny > , rsa_padding : pyo3 :: Bound < 'p , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < common :: AlgorithmIdentifier < 'static > > { let key_type = x509 :: sign :: identify_key_type (py , private_key . clone ()) ? ; let has_pss_padding = rsa_padding . is_instance (& types :: PSS . get (py) ?) ? ; if key_type == x509 :: sign :: KeyType :: Rsa && ! has_pss_padding { Ok (common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Rsa (Some (())) , }) } else { x509 :: sign :: compute_signature_algorithm (py , private_key , hash_algorithm , rsa_padding) } }
};
}
