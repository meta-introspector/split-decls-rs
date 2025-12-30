// Generated macro for impl_211 (impl)
macro_rules! Depcrateimpl_211 {
() => {
// Module: crate
// Provides: {"impl_211"}
// Dependencies: {}
# [doc = " ECDSA `AlgorithmIdentifier` which identifies the digest used by default"] # [doc = " with the `Signer` and `Verifier` traits."] # [cfg (feature = "pkcs8")] impl < C > AssociatedAlgorithmIdentifier for Signature < C > where C : EcdsaCurve , Self : AssociatedOid , { type Params = AnyRef < 'static > ; const ALGORITHM_IDENTIFIER : AlgorithmIdentifierRef < 'static > = AlgorithmIdentifierRef { oid : Self :: OID , parameters : None , } ; }
};
}
