// Generated macro for Base64UrlSafeNoPadding (struct)
macro_rules! Depcrate_base64Base64UrlSafeNoPadding {
() => {
// Module: crate::base64
// Provides: {"Base64UrlSafeNoPadding"}
// Dependencies: {}
# [doc = " URL-safe Base64 encoder and decoder without padding."] # [doc = ""] # [doc = " This implementation follows the URL-safe Base64 encoding variant as defined in RFC 4648,"] # [doc = " but omits padding characters ('='). This is particularly useful for URLs, where the"] # [doc = " padding character may need to be percent-encoded."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ct_codecs::{Base64UrlSafeNoPadding, Encoder, Decoder};"] # [doc = ""] # [doc = " fn example() -> Result<(), ct_codecs::Error> {"] # [doc = "     let data = b\"Hello, world!\";"] # [doc = "     let encoded = Base64UrlSafeNoPadding::encode_to_string(data)?;"] # [doc = "     assert_eq!(encoded, \"SGVsbG8sIHdvcmxkIQ\");"] # [doc = ""] # [doc = "     // With binary data containing characters that would be escaped in URLs"] # [doc = "     let binary_data = &[251, 239, 190, 222];"] # [doc = "     let encoded = Base64UrlSafeNoPadding::encode_to_string(binary_data)?;"] # [doc = "     assert_eq!(encoded, \"----3g\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " # example().unwrap();"] # [doc = " ```"] pub struct Base64UrlSafeNoPadding ;
};
}
