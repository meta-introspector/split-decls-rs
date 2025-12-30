// Generated macro for RevokedCertificates (type)
macro_rules! Depcrate_crlRevokedCertificates {
() => {
// Module: crate::crl
// Provides: {"RevokedCertificates"}
// Dependencies: {}
pub type RevokedCertificates < 'a > = Option < common :: Asn1ReadableOrWritable < asn1 :: SequenceOf < 'a , RevokedCertificate < 'a > > , asn1 :: SequenceOfWriter < 'a , RevokedCertificate < 'a > , Vec < RevokedCertificate < 'a > > > , > , > ;
};
}
