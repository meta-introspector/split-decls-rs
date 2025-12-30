// Generated macro for DigestAlgorithmIdentifiers (type)
macro_rules! Depcrate_signed_dataDigestAlgorithmIdentifiers {
() => {
// Module: crate::signed_data
// Provides: {"DigestAlgorithmIdentifiers"}
// Dependencies: {}
# [doc = " The `DigestAlgorithmIdentifiers` type is defined in [RFC 5652 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " DigestAlgorithmIdentifiers ::= SET OF DigestAlgorithmIdentifier"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc5652#section-5.1"] pub type DigestAlgorithmIdentifiers = SetOfVec < AlgorithmIdentifierOwned > ;
};
}
