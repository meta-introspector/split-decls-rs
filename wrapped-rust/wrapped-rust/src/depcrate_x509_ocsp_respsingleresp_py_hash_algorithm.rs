// Generated macro for singleresp_py_hash_algorithm (function)
macro_rules! Depcrate_x509_ocsp_respsingleresp_py_hash_algorithm {
() => {
// Module: crate::x509::ocsp_resp
// Provides: {"singleresp_py_hash_algorithm"}
// Dependencies: {}
fn singleresp_py_hash_algorithm < 'p > (resp : & ocsp_resp :: SingleResponse < '_ > , py : pyo3 :: Python < 'p > ,) -> Result < pyo3 :: Bound < 'p , pyo3 :: PyAny > , CryptographyError > { match ocsp :: ALGORITHM_PARAMETERS_TO_HASH . get (& resp . cert_id . hash_algorithm . params) { Some (alg_name) => Ok (types :: HASHES_MODULE . get (py) ? . getattr (* alg_name) ? . call0 () ?) , None => Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err (format ! ("Signature algorithm OID: {} not recognized" , resp . cert_id . hash_algorithm . oid ())) ,)) , } }
};
}
