// Generated macro for match_digest_type (function)
macro_rules! Depcrate_digestmatch_digest_type {
() => {
// Module: crate::digest
// Provides: {"match_digest_type"}
// Dependencies: {}
# [doc = " Match digest types for `EVP_MD` functions."] pub (crate) fn match_digest_type (algorithm_id : & AlgorithmID) -> ConstPointer < '_ , EVP_MD > { unsafe { ConstPointer :: new_static (match algorithm_id { AlgorithmID :: SHA1 => EVP_sha1 () , AlgorithmID :: SHA224 => EVP_sha224 () , AlgorithmID :: SHA256 => EVP_sha256 () , AlgorithmID :: SHA384 => EVP_sha384 () , AlgorithmID :: SHA512 => EVP_sha512 () , AlgorithmID :: SHA512_256 => EVP_sha512_256 () , AlgorithmID :: SHA3_256 => EVP_sha3_256 () , AlgorithmID :: SHA3_384 => EVP_sha3_384 () , AlgorithmID :: SHA3_512 => EVP_sha3_512 () , }) . unwrap_or_else (| () | panic ! ("Digest algorithm not found: {algorithm_id:?}")) } }
};
}
