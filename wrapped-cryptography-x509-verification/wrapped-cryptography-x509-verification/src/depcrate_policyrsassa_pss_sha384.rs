// Generated macro for RSASSA_PSS_SHA384 (static)
macro_rules! Depcrate_policyRSASSA_PSS_SHA384 {
() => {
// Module: crate::policy
// Provides: {"RSASSA_PSS_SHA384"}
// Dependencies: {}
static RSASSA_PSS_SHA384 : LazyLock < AlgorithmIdentifier < '_ > > = LazyLock :: new (| | AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : AlgorithmParameters :: RsaPss (Some (Box :: new (RsaPssParameters { hash_algorithm : PSS_SHA384_HASH_ALG , mask_gen_algorithm : PSS_SHA384_MASK_GEN_ALG , salt_length : 48 , _trailer_field : None , }))) , }) ;
};
}
