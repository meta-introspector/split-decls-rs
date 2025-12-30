// Generated macro for CertificateRevocationList (struct)
macro_rules! Depcrate_crlCertificateRevocationList {
() => {
// Module: crate::crl
// Provides: {"CertificateRevocationList"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash)] pub struct CertificateRevocationList < 'a > { pub tbs_cert_list : TBSCertList < 'a > , pub signature_algorithm : common :: AlgorithmIdentifier < 'a > , pub signature_value : asn1 :: BitString < 'a > , }
};
}
