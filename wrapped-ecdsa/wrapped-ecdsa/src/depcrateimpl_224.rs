// Generated macro for impl_224 (impl)
macro_rules! Depcrateimpl_224 {
() => {
// Module: crate
// Provides: {"impl_224"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > DynAssociatedAlgorithmIdentifier for SignatureWithOid < C > where C : EcdsaCurve , { fn algorithm_identifier (& self) -> spki :: Result < AlgorithmIdentifierOwned > { Ok (AlgorithmIdentifierOwned { oid : self . oid , parameters : None , }) } }
};
}
