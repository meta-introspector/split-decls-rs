// Generated macro for RSASSA_PSS_SHA512 (static)
macro_rules! Depcrate_policyRSASSA_PSS_SHA512 {
() => {
// Module: crate::policy
// Provides: {"RSASSA_PSS_SHA512"}
// Dependencies: {}
static RSASSA_PSS_SHA512 : LazyLock < AlgorithmIdentifier < '_ > > = LazyLock :: new (| | AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : AlgorithmParameters :: RsaPss (Some (Box :: new (RsaPssParameters { hash_algorithm : PSS_SHA512_HASH_ALG , mask_gen_algorithm : PSS_SHA512_MASK_GEN_ALG , salt_length : 64 , _trailer_field : None , }))) , }) ;
};
}
