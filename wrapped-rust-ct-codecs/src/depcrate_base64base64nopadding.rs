// Generated macro for Base64NoPadding (struct)
macro_rules! Depcrate_base64Base64NoPadding {
() => {
// Module: crate::base64
// Provides: {"Base64NoPadding"}
// Dependencies: {}
# [doc = " Standard Base64 encoder and decoder without padding."] # [doc = ""] # [doc = " This implementation follows the standard Base64 encoding as defined in RFC 4648,"] # [doc = " but omits padding characters ('=')."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ct_codecs::{Base64NoPadding, Encoder, Decoder};"] # [doc = ""] # [doc = " fn example() -> Result<(), ct_codecs::Error> {"] # [doc = "     let data = b\"Hello, world!\";"] # [doc = "     let encoded = Base64NoPadding::encode_to_string(data)?;"] # [doc = "     assert_eq!(encoded, \"SGVsbG8sIHdvcmxkIQ\");"] # [doc = ""] # [doc = "     let decoded = Base64NoPadding::decode_to_vec(&encoded, None)?;"] # [doc = "     assert_eq!(decoded, data);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " # example().unwrap();"] # [doc = " ```"] pub struct Base64NoPadding ;
};
}
