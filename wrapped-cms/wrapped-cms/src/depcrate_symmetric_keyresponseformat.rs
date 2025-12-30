// Generated macro for ResponseFormat (struct)
macro_rules! Depcrate_symmetric_keyResponseFormat {
() => {
// Module: crate::symmetric_key
// Provides: {"ResponseFormat"}
// Dependencies: {}
# [doc = " The `ResponseFormat` type is defined in [RFC 6031 Section 3.2.7]."] # [doc = ""] # [doc = " ```text"] # [doc = "    ResponseFormat ::= SEQUENCE {"] # [doc = "      encoding     Encoding,"] # [doc = "      length       INTEGER (0..MAX),"] # [doc = "      checkDigit   BOOLEAN DEFAULT FALSE,"] # [doc = "      ... }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.7]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.7"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct ResponseFormat { pub encoding : Encoding , pub length : u32 , # [asn1 (default = "Default::default")] pub check_digit : bool , }
};
}
