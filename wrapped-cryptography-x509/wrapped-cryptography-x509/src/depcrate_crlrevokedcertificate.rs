// Generated macro for RevokedCertificate (struct)
macro_rules! Depcrate_crlRevokedCertificate {
() => {
// Module: crate::crl
// Provides: {"RevokedCertificate"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone)] pub struct RevokedCertificate < 'a > { pub user_certificate : SerialNumber < 'a > , pub revocation_date : common :: Time , pub raw_crl_entry_extensions : Option < extensions :: RawExtensions < 'a > > , }
};
}
