// Generated macro for CertID (struct)
macro_rules! Depcrate_ocsp_reqCertID {
() => {
// Module: crate::ocsp_req
// Provides: {"CertID"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct CertID < 'a > { pub hash_algorithm : common :: AlgorithmIdentifier < 'a > , pub issuer_name_hash : & 'a [u8] , pub issuer_key_hash : & 'a [u8] , pub serial_number : asn1 :: BigInt < 'a > , }
};
}
