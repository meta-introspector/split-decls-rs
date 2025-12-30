// Generated macro for SubjectPublicKeyInfo (struct)
macro_rules! Depcrate_commonSubjectPublicKeyInfo {
() => {
// Module: crate::common
// Provides: {"SubjectPublicKeyInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , PartialEq , Eq , Clone)] pub struct SubjectPublicKeyInfo < 'a > { pub algorithm : AlgorithmIdentifier < 'a > , pub subject_public_key : asn1 :: BitString < 'a > , }
};
}
