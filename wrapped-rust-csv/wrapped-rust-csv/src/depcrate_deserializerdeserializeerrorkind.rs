// Generated macro for DeserializeErrorKind (enum)
macro_rules! Depcrate_deserializerDeserializeErrorKind {
() => {
// Module: crate::deserializer
// Provides: {"DeserializeErrorKind"}
// Dependencies: {}
# [doc = " The type of a Serde deserialization error."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum DeserializeErrorKind { # [doc = " A generic Serde deserialization error."] Message (String) , # [doc = " A generic Serde unsupported error."] Unsupported (String) , # [doc = " This error occurs when a Rust type expects to decode another field"] # [doc = " from a row, but no more fields exist."] UnexpectedEndOfRow , # [doc = " This error occurs when UTF-8 validation on a field fails. UTF-8"] # [doc = " validation is only performed when the Rust type requires it (e.g.,"] # [doc = " a `String` or `&str` type)."] InvalidUtf8 (str :: Utf8Error) , # [doc = " This error occurs when a boolean value fails to parse."] ParseBool (str :: ParseBoolError) , # [doc = " This error occurs when an integer value fails to parse."] ParseInt (num :: ParseIntError) , # [doc = " This error occurs when a float value fails to parse."] ParseFloat (num :: ParseFloatError) , }
};
}
