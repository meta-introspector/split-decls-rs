// Generated macro for impl_140 (impl)
macro_rules! Depcrate_pkcs8impl_140 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_140"}
// Dependencies: {}
impl < P > TryFrom < SubjectPublicKeyInfoRef < '_ > > for VerifyingKey < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Error = spki :: Error ; fn try_from (spki : SubjectPublicKeyInfoRef < '_ >) -> spki :: Result < Self > { spki . algorithm . assert_algorithm_oid (P :: ALGORITHM_IDENTIFIER . oid) ? ; Ok (Self :: decode (& EncodedVerifyingKey :: < P > :: try_from (spki . subject_public_key . as_bytes () . ok_or_else (| | der :: Tag :: BitString . value_error () . to_error ()) ? ,) . map_err (| _ | :: pkcs8 :: Error :: KeyMalformed) ? ,)) } }
};
}
