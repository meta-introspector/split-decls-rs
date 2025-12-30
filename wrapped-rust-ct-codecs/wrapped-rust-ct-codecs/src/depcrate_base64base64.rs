// Generated macro for Base64 (struct)
macro_rules! Depcrate_base64Base64 {
() => {
// Module: crate::base64
// Provides: {"Base64"}
// Dependencies: {}
# [doc = " Standard Base64 encoder and decoder with padding."] # [doc = ""] # [doc = " This implementation follows the standard Base64 encoding as defined in RFC 4648,"] # [doc = " and includes padding characters ('=') when needed."] # [doc = ""] # [doc = " # Standard Base64 Alphabet"] # [doc = ""] # [doc = " The standard Base64 alphabet uses characters:"] # [doc = " - 'A' to 'Z' (26 characters)"] # [doc = " - 'a' to 'z' (26 characters)"] # [doc = " - '0' to '9' (10 characters)"] # [doc = " - '+' and '/' (2 characters)"] # [doc = " - '=' (padding character)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ct_codecs::{Base64, Encoder, Decoder};"] # [doc = ""] # [doc = " fn example() -> Result<(), ct_codecs::Error> {"] # [doc = "     let data = b\"Hello, world!\";"] # [doc = "     let encoded = Base64::encode_to_string(data)?;"] # [doc = "     assert_eq!(encoded, \"SGVsbG8sIHdvcmxkIQ==\");"] # [doc = ""] # [doc = "     let decoded = Base64::decode_to_vec(&encoded, None)?;"] # [doc = "     assert_eq!(decoded, data);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " # example().unwrap();"] # [doc = " ```"] pub struct Base64 ;
};
}
