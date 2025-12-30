// Generated macro for OneSymmetricKey (struct)
macro_rules! Depcrate_symmetric_keyOneSymmetricKey {
() => {
// Module: crate::symmetric_key
// Provides: {"OneSymmetricKey"}
// Dependencies: {}
# [doc = " The `OneSymmetricKey` type is defined in [RFC 6031 Section 2.0]."] # [doc = ""] # [doc = " ```text"] # [doc = "      OneSymmetricKey ::= SEQUENCE {"] # [doc = "        sKeyAttrs  SEQUENCE SIZE (1..MAX) OF Attribute"] # [doc = "                                       {{ SKeyAttributes }}  OPTIONAL,"] # [doc = "        sKey       OCTET STRING OPTIONAL }"] # [doc = "        ( WITH COMPONENTS { ..., sKeyAttrs PRESENT } |"] # [doc = "          WITH COMPONENTS { ..., sKey PRESENT } )"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 2.0]: https://datatracker.ietf.org/doc/html/rfc6031#section-2"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct OneSymmetricKey { pub s_key_attrs : Vec < Attribute > , # [asn1 (optional = "true")] pub s_key : Option < OctetString > , }
};
}
