// Generated macro for Csr (struct)
macro_rules! Depcrate_csrCsr {
() => {
// Module: crate::csr
// Provides: {"Csr"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct Csr < 'a > { pub csr_info : CertificationRequestInfo < 'a > , pub signature_alg : common :: AlgorithmIdentifier < 'a > , pub signature : asn1 :: BitString < 'a > , }
};
}
