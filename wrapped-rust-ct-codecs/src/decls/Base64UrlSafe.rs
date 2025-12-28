macro_rules! deps {
    () => {
        Decoder!();
        Encoder!();
        Base64!();
    };
}

macro_rules! Base64UrlSafe {
    () => {
        deps!();
        # [doc = " URL-safe Base64 encoder and decoder with padding."] # [doc = ""] # [doc = " This implementation follows the URL-safe Base64 encoding variant as defined in RFC 4648."] # [doc = " It replaces the '+' and '/' characters with '-' and '_' to make the output URL and"] # [doc = " filename safe. Padding characters ('=') are included when needed."] # [doc = ""] # [doc = " # URL-safe Base64 Alphabet"] # [doc = ""] # [doc = " The URL-safe Base64 alphabet uses characters:"] # [doc = " - 'A' to 'Z' (26 characters)"] # [doc = " - 'a' to 'z' (26 characters)"] # [doc = " - '0' to '9' (10 characters)"] # [doc = " - '-' and '_' (2 characters)"] # [doc = " - '=' (padding character)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ct_codecs::{Base64UrlSafe, Encoder, Decoder};"] # [doc = ""] # [doc = " fn example() -> Result<(), ct_codecs::Error> {"] # [doc = "     let data = b\"Hello, world!\";"] # [doc = "     let encoded = Base64UrlSafe::encode_to_string(data)?;"] # [doc = "     assert_eq!(encoded, \"SGVsbG8sIHdvcmxkIQ==\");"] # [doc = ""] # [doc = "     // If the input contains characters that would be escaped in URLs"] # [doc = "     let binary_data = &[251, 239, 190, 222];"] # [doc = "     let encoded = Base64UrlSafe::encode_to_string(binary_data)?;"] # [doc = "     assert_eq!(encoded, \"----3g==\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " # example().unwrap();"] # [doc = " ```"] pub struct Base64UrlSafe ;
    };
}

Base64UrlSafe!()