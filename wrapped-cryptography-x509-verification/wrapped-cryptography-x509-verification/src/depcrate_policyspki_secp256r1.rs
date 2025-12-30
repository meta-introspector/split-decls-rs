// Generated macro for SPKI_SECP256R1 (const)
macro_rules! Depcrate_policySPKI_SECP256R1 {
() => {
// Module: crate::policy
// Provides: {"SPKI_SECP256R1"}
// Dependencies: {}
const SPKI_SECP256R1 : AlgorithmIdentifier < '_ > = AlgorithmIdentifier { oid : asn1 :: DefinedByMarker :: marker () , params : AlgorithmParameters :: Ec (EcParameters :: NamedCurve (EC_SECP256R1)) , } ;
};
}
