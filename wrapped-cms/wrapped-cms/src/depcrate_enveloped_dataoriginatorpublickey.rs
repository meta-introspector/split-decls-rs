// Generated macro for OriginatorPublicKey (struct)
macro_rules! Depcrate_enveloped_dataOriginatorPublicKey {
() => {
// Module: crate::enveloped_data
// Provides: {"OriginatorPublicKey"}
// Dependencies: {}
# [doc = " The `OriginatorPublicKey` type is defined in [RFC 5652 Section 6.2.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   OriginatorPublicKey ::= SEQUENCE {"] # [doc = "       algorithm AlgorithmIdentifier {PUBLIC-KEY, {OriginatorKeySet}},"] # [doc = "       publicKey BIT STRING }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OriginatorPublicKey { pub algorithm : AlgorithmIdentifierOwned , pub public_key : BitString , }
};
}
