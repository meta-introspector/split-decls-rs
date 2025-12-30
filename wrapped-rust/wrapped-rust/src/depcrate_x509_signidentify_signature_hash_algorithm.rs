// Generated macro for identify_signature_hash_algorithm (function)
macro_rules! Depcrate_x509_signidentify_signature_hash_algorithm {
() => {
// Module: crate::x509::sign
// Provides: {"identify_signature_hash_algorithm"}
// Dependencies: {}
pub (crate) fn identify_signature_hash_algorithm < 'p > (py : pyo3 :: Python < 'p > , signature_algorithm : & common :: AlgorithmIdentifier < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let sig_oids_to_hash = types :: SIG_OIDS_TO_HASH . get (py) ? ; match & signature_algorithm . params { common :: AlgorithmParameters :: RsaPss (opt_pss) => { let pss = opt_pss . as_ref () . ok_or_else (| | { pyo3 :: exceptions :: PyValueError :: new_err ("Invalid RSA PSS parameters") }) ? ; hash_oid_py_hash (py , pss . hash_algorithm . oid () . clone ()) } _ => { let py_sig_alg_oid = oid_to_py_oid (py , signature_algorithm . oid ()) ? ; let hash_alg = sig_oids_to_hash . get_item (py_sig_alg_oid) ; match hash_alg { Ok (data) => Ok (data) , Err (_) => Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err (format ! ("Signature algorithm OID: {} not recognized" , signature_algorithm . oid ())) ,)) , } } } }
};
}
