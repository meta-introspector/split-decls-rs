// Generated macro for RSASSA_PSS_SHA256 (static)
macro_rules! Depcrate_policyRSASSA_PSS_SHA256 {
() => {
// Module: crate::policy
// Provides: {"RSASSA_PSS_SHA256"}
// Dependencies: {}
static RSASSA_PSS_SHA256 : LazyLock < AlgorithmIdentifier < '_ > > = LazyLock :: new (| | AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : AlgorithmParameters :: RsaPss (Some (Box :: new (RsaPssParameters { hash_algorithm : PSS_SHA256_HASH_ALG , mask_gen_algorithm : PSS_SHA256_MASK_GEN_ALG , salt_length : 32 , _trailer_field : None , }))) , }) ;
};
}
