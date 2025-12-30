// Generated macro for ALGORITHM_PARAMETERS_TO_HASH (static)
macro_rules! Depcrate_x509_ocspALGORITHM_PARAMETERS_TO_HASH {
() => {
// Module: crate::x509::ocsp
// Provides: {"ALGORITHM_PARAMETERS_TO_HASH"}
// Dependencies: {}
pub (crate) static ALGORITHM_PARAMETERS_TO_HASH : LazyLock < HashMap < common :: AlgorithmParameters < '_ > , & str > , > = LazyLock :: new (| | { let mut h = HashMap :: new () ; h . insert (common :: AlgorithmParameters :: Sha1 (None) , "SHA1") ; h . insert (common :: AlgorithmParameters :: Sha1 (Some (())) , "SHA1") ; h . insert (common :: AlgorithmParameters :: Sha224 (None) , "SHA224") ; h . insert (common :: AlgorithmParameters :: Sha224 (Some (())) , "SHA224") ; h . insert (common :: AlgorithmParameters :: Sha256 (None) , "SHA256") ; h . insert (common :: AlgorithmParameters :: Sha256 (Some (())) , "SHA256") ; h . insert (common :: AlgorithmParameters :: Sha384 (None) , "SHA384") ; h . insert (common :: AlgorithmParameters :: Sha384 (Some (())) , "SHA384") ; h . insert (common :: AlgorithmParameters :: Sha512 (None) , "SHA512") ; h . insert (common :: AlgorithmParameters :: Sha512 (Some (())) , "SHA512") ; h }) ;
};
}
