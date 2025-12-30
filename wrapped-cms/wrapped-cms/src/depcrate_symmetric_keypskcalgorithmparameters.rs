// Generated macro for PSKCAlgorithmParameters (struct)
macro_rules! Depcrate_symmetric_keyPSKCAlgorithmParameters {
() => {
// Module: crate::symmetric_key
// Provides: {"PSKCAlgorithmParameters"}
// Dependencies: {}
# [doc = " The `PSKCAlgorithmParameters` type is defined in [RFC 6031 Section 3.2.7]."] # [doc = ""] # [doc = "  ```text"] # [doc = "    PSKCAlgorithmParameters ::= CHOICE {"] # [doc = "      suite                UTF8String,"] # [doc = "      challengeFormat  [0] ChallengeFormat,"] # [doc = "      responseFormat   [1] ResponseFormat,"] # [doc = "      ... }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.7]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.7"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct PSKCAlgorithmParameters { pub suite : String , pub challenge_format : ChallengeFormat , pub response_format : ResponseFormat , }
};
}
