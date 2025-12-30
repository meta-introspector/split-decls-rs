// Generated macro for SubsequentMessage (enum)
macro_rules! Depcrate_popSubsequentMessage {
() => {
// Module: crate::pop
// Provides: {"SubsequentMessage"}
// Dependencies: {}
# [doc = " The `SubsequentMessage` type is defined in [RFC 4211 Section 4.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   SubsequentMessage ::= INTEGER {"] # [doc = "       encrCert (0),"] # [doc = "       challengeResp (1) }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.2]: https://www.rfc-editor.org/rfc/rfc4211#section-4.2"] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [allow (missing_docs)] pub enum SubsequentMessage { EncrCert = 0 , ChallengeResp = 1 , }
};
}
