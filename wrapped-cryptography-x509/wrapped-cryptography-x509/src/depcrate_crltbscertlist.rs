// Generated macro for TBSCertList (struct)
macro_rules! Depcrate_crlTBSCertList {
() => {
// Module: crate::crl
// Provides: {"TBSCertList"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash)] pub struct TBSCertList < 'a > { pub version : Option < u8 > , pub signature : common :: AlgorithmIdentifier < 'a > , pub issuer : name :: Name < 'a > , pub this_update : common :: Time , pub next_update : Option < common :: Time > , pub revoked_certificates : RevokedCertificates < 'a > , # [explicit (0)] pub raw_crl_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
