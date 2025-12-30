// Generated macro for OriginatorIdentifierOrKey (enum)
macro_rules! Depcrate_enveloped_dataOriginatorIdentifierOrKey {
() => {
// Module: crate::enveloped_data
// Provides: {"OriginatorIdentifierOrKey"}
// Dependencies: {}
# [doc = " The `OriginatorIdentifierOrKey` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OriginatorIdentifierOrKey ::= CHOICE {"] # [doc = "       issuerAndSerialNumber IssuerAndSerialNumber,"] # [doc = "       subjectKeyIdentifier [0] SubjectKeyIdentifier,"] # [doc = "       originatorKey [1] OriginatorPublicKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum OriginatorIdentifierOrKey { IssuerAndSerialNumber (IssuerAndSerialNumber) , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT")] SubjectKeyIdentifier (SubjectKeyIdentifier) , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true")] OriginatorKey (OriginatorPublicKey) , }
};
}
