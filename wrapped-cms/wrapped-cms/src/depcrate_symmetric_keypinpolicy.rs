// Generated macro for PINPolicy (struct)
macro_rules! Depcrate_symmetric_keyPINPolicy {
() => {
// Module: crate::symmetric_key
// Provides: {"PINPolicy"}
// Dependencies: {}
# [doc = " The `PINPolicy` type is defined in [RFC 6031 Section 3.3.5]"] # [doc = ""] # [doc = " ```text"] # [doc = "    PINPolicy ::= SEQUENCE {"] # [doc = "      pinKeyId          [0] UTF8String OPTIONAL,"] # [doc = "      pinUsageMode      [1] PINUsageMode,"] # [doc = "      maxFailedAttempts [2] INTEGER (0..MAX) OPTIONAL,"] # [doc = "      minLength         [3] INTEGER (0..MAX) OPTIONAL,"] # [doc = "      maxLength         [4] INTEGER (0..MAX) OPTIONAL,"] # [doc = "      pinEncoding       [5] Encoding OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.3.5]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.3.5"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct PINPolicy { # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub pin_key_id : Option < String > , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT")] pub pin_usage_mode : PINUsageMode , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT" , optional = "true")] pub max_failed_attempts : Option < u32 > , # [asn1 (context_specific = "3" , tag_mode = "IMPLICIT" , optional = "true")] pub min_length : Option < u32 > , # [asn1 (context_specific = "4" , tag_mode = "IMPLICIT" , optional = "true")] pub max_length : Option < u32 > , # [asn1 (context_specific = "5" , tag_mode = "IMPLICIT" , optional = "true")] pub pin_encoding : Option < Encoding > , }
};
}
