// Generated macro for identify_hash_type (function)
macro_rules! Depcrate_x509_signidentify_hash_type {
() => {
// Module: crate::x509::sign
// Provides: {"identify_hash_type"}
// Dependencies: {}
fn identify_hash_type (py : pyo3 :: Python < '_ > , hash_algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < HashType > { if hash_algorithm . is_none () { return Ok (HashType :: None) ; } if ! hash_algorithm . is_instance (& types :: HASH_ALGORITHM . get (py) ?) ? { return Err (pyo3 :: exceptions :: PyTypeError :: new_err ("Algorithm must be a registered hash algorithm." ,)) ; } match & * hash_algorithm . getattr (pyo3 :: intern ! (py , "name")) ? . extract :: < pyo3 :: pybacked :: PyBackedStr > () ? { "sha224" => Ok (HashType :: Sha224) , "sha256" => Ok (HashType :: Sha256) , "sha384" => Ok (HashType :: Sha384) , "sha512" => Ok (HashType :: Sha512) , "sha3-224" => Ok (HashType :: Sha3_224) , "sha3-256" => Ok (HashType :: Sha3_256) , "sha3-384" => Ok (HashType :: Sha3_384) , "sha3-512" => Ok (HashType :: Sha3_512) , name => Err (exceptions :: UnsupportedAlgorithm :: new_err (format ! ("Hash algorithm {name:?} not supported for signatures"))) , } }
};
}
