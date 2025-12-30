// Generated macro for TbsCertificate (struct)
macro_rules! Depcrate_certificateTbsCertificate {
() => {
// Module: crate::certificate
// Provides: {"TbsCertificate"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , PartialEq , Eq , Clone)] pub struct TbsCertificate < 'a > { # [explicit (0)] # [default (0)] pub version : u8 , pub serial : SerialNumber < 'a > , pub signature_alg : common :: AlgorithmIdentifier < 'a > , pub issuer : name :: Name < 'a > , pub validity : Validity , pub subject : name :: Name < 'a > , pub spki : common :: WithTlv < 'a , common :: SubjectPublicKeyInfo < 'a > > , # [implicit (1)] pub issuer_unique_id : Option < asn1 :: BitString < 'a > > , # [implicit (2)] pub subject_unique_id : Option < asn1 :: BitString < 'a > > , # [explicit (3)] pub raw_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
