// Generated macro for hash_oid_py_hash (function)
macro_rules! Depcrate_x509_signhash_oid_py_hash {
() => {
// Module: crate::x509::sign
// Provides: {"hash_oid_py_hash"}
// Dependencies: {}
fn hash_oid_py_hash (py : pyo3 :: Python < '_ > , oid : asn1 :: ObjectIdentifier ,) -> CryptographyResult < pyo3 :: Bound < '_ , pyo3 :: PyAny > > { match HASH_OIDS_TO_HASH . get (& oid) { Some (alg_name) => Ok (types :: HASHES_MODULE . get (py) ? . getattr (* alg_name) ? . call0 () ?) , None => Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err (format ! ("Signature algorithm OID: {} not recognized" , & oid)) ,)) , } }
};
}
