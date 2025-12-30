// Generated macro for Certificate (struct)
macro_rules! Depcrate_certificateCertificate {
() => {
// Module: crate::certificate
// Provides: {"Certificate"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , PartialEq , Eq , Clone)] pub struct Certificate < 'a > { pub tbs_cert : TbsCertificate < 'a > , pub signature_alg : common :: AlgorithmIdentifier < 'a > , pub signature : asn1 :: BitString < 'a > , }
};
}
