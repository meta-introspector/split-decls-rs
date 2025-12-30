// Generated macro for EncodingError (enum)
macro_rules! Depcrate_encodingEncodingError {
() => {
// Module: crate::encoding
// Provides: {"EncodingError"}
// Dependencies: {}
# [doc = " An error when decoding or encoding"] # [doc = ""] # [doc = " If feature [`encoding`] is disabled, the [`EncodingError`] is always [`EncodingError::Utf8`]"] # [doc = ""] # [doc = " [`encoding`]: ../index.html#encoding"] # [derive (Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum EncodingError { # [doc = " Input was not valid UTF-8"] Utf8 (Utf8Error) , # [doc = " Input did not adhere to the given encoding"] # [cfg (feature = "encoding")] Other (& 'static Encoding) , }
};
}
