// Generated macro for CertificationRequestInfo (struct)
macro_rules! Depcrate_csrCertificationRequestInfo {
() => {
// Module: crate::csr
// Provides: {"CertificationRequestInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct CertificationRequestInfo < 'a > { pub version : u8 , pub subject : name :: Name < 'a > , pub spki : common :: WithTlv < 'a , common :: SubjectPublicKeyInfo < 'a > > , # [implicit (0 , required)] pub attributes : Attributes < 'a > , }
};
}
