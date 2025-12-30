// Generated macro for CertId (struct)
macro_rules! Depcrate_controlsCertId {
() => {
// Module: crate::controls
// Provides: {"CertId"}
// Dependencies: {}
# [doc = " The `CertId` control is defined in [RFC 4211 Section 6.5]."] # [doc = ""] # [doc = " ```text"] # [doc = "   CertId ::= SEQUENCE {"] # [doc = "       issuer           GeneralName,"] # [doc = "       serialNumber     INTEGER }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6.5]: https://www.rfc-editor.org/rfc/rfc4211#section-6.5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertId < P : Profile = Rfc5280 > { pub issuer : GeneralName , pub serial_number : SerialNumber < P > , }
};
}
