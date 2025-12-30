// Generated macro for RawCRLIterator (type)
macro_rules! Depcrate_x509_crlRawCRLIterator {
() => {
// Module: crate::x509::crl
// Provides: {"RawCRLIterator"}
// Dependencies: {}
type RawCRLIterator < 'a > = Option < asn1 :: SequenceOf < 'a , crl :: RevokedCertificate < 'a > > > ;
};
}
