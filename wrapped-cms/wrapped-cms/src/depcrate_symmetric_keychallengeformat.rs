// Generated macro for ChallengeFormat (struct)
macro_rules! Depcrate_symmetric_keyChallengeFormat {
() => {
// Module: crate::symmetric_key
// Provides: {"ChallengeFormat"}
// Dependencies: {}
# [doc = " The `ChallengeFormat` type is defined in [RFC 6031 Section 3.2.7]."] # [doc = ""] # [doc = " ```text"] # [doc = "    ChallengeFormat ::= SEQUENCE {"] # [doc = "      encoding    Encoding,"] # [doc = "      checkDigit  BOOLEAN DEFAULT FALSE,"] # [doc = "      min         INTEGER (0..MAX),"] # [doc = "      max         INTEGER (0..MAX),"] # [doc = "      ... }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.7]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.7"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct ChallengeFormat { pub encoding : Encoding , # [asn1 (default = "Default::default")] pub check_digit : bool , pub min : der :: asn1 :: Int , pub max : der :: asn1 :: Int , }
};
}
