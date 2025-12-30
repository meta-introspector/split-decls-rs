// Generated macro for identify_alg_params_for_hash_type (function)
macro_rules! Depcrate_x509_signidentify_alg_params_for_hash_type {
() => {
// Module: crate::x509::sign
// Provides: {"identify_alg_params_for_hash_type"}
// Dependencies: {}
fn identify_alg_params_for_hash_type (hash_type : HashType ,) -> pyo3 :: PyResult < common :: AlgorithmParameters < 'static > > { match hash_type { HashType :: Sha224 => Ok (common :: AlgorithmParameters :: Sha224 (Some (()))) , HashType :: Sha256 => Ok (common :: AlgorithmParameters :: Sha256 (Some (()))) , HashType :: Sha384 => Ok (common :: AlgorithmParameters :: Sha384 (Some (()))) , HashType :: Sha512 => Ok (common :: AlgorithmParameters :: Sha512 (Some (()))) , HashType :: Sha3_224 => Ok (common :: AlgorithmParameters :: Sha3_224Nist (Some (()))) , HashType :: Sha3_256 => Ok (common :: AlgorithmParameters :: Sha3_256Nist (Some (()))) , HashType :: Sha3_384 => Ok (common :: AlgorithmParameters :: Sha3_384Nist (Some (()))) , HashType :: Sha3_512 => Ok (common :: AlgorithmParameters :: Sha3_512Nist (Some (()))) , HashType :: None => Err (pyo3 :: exceptions :: PyTypeError :: new_err ("Algorithm must be a registered hash algorithm, not None." ,)) , } }
};
}
