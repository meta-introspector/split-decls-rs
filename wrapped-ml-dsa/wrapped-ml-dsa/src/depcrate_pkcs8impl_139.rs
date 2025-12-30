// Generated macro for impl_139 (impl)
macro_rules! Depcrate_pkcs8impl_139 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P > EncodePublicKey for VerifyingKey < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { fn to_public_key_der (& self) -> spki :: Result < der :: Document > { let public_key = self . encode () ; let subject_public_key = BitStringRef :: new (0 , & public_key) ? ; SubjectPublicKeyInfo { algorithm : P :: ALGORITHM_IDENTIFIER , subject_public_key , } . try_into () } }
};
}
