// Generated macro for CertificateSet (struct)
macro_rules! Depcrate_signed_dataCertificateSet {
() => {
// Module: crate::signed_data
// Provides: {"CertificateSet"}
// Dependencies: {}
# [doc = " CertificateSet structure as defined in [RFC 5652 Section 10.2.3]."] # [doc = ""] # [doc = " ```text"] # [doc = "   CertificateSet ::= SET OF CertificateChoices"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.3]: https://datatracker.ietf.org/doc/html/rfc5652#section-10.2.3"] # [derive (Clone , Eq , PartialEq , Debug)] pub struct CertificateSet (pub SetOfVec < CertificateChoices >) ;
};
}
