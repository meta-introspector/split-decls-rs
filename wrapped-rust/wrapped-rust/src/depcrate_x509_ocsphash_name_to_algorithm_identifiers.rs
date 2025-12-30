// Generated macro for HASH_NAME_TO_ALGORITHM_IDENTIFIERS (static)
macro_rules! Depcrate_x509_ocspHASH_NAME_TO_ALGORITHM_IDENTIFIERS {
() => {
// Module: crate::x509::ocsp
// Provides: {"HASH_NAME_TO_ALGORITHM_IDENTIFIERS"}
// Dependencies: {}
pub (crate) static HASH_NAME_TO_ALGORITHM_IDENTIFIERS : LazyLock < HashMap < & str , common :: AlgorithmIdentifier < '_ > > , > = LazyLock :: new (| | { let mut h = HashMap :: new () ; h . insert ("sha1" , common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Sha1 (Some (())) , } ,) ; h . insert ("sha224" , common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Sha224 (Some (())) , } ,) ; h . insert ("sha256" , common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Sha256 (Some (())) , } ,) ; h . insert ("sha384" , common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Sha384 (Some (())) , } ,) ; h . insert ("sha512" , common :: AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : common :: AlgorithmParameters :: Sha512 (Some (())) , } ,) ; h }) ;
};
}
