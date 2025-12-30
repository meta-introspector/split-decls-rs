// Generated macro for IssuerAndSerialNumber (struct)
macro_rules! Depcrate_certIssuerAndSerialNumber {
() => {
// Module: crate::cert
// Provides: {"IssuerAndSerialNumber"}
// Dependencies: {}
# [doc = " IssuerAndSerialNumber structure as defined in [RFC 5652 Section 10.2.4]."] # [doc = ""] # [doc = " ```text"] # [doc = " IssuerAndSerialNumber ::= SEQUENCE {"] # [doc = "   issuer Name,"] # [doc = "   serialNumber CertificateSerialNumber }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 10.2.4]: https://datatracker.ietf.org/doc/html/rfc5652#section-10.2.4"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct IssuerAndSerialNumber { pub issuer : Name , pub serial_number : SerialNumber , }
};
}
