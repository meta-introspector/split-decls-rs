// Generated macro for Utf8Error (struct)
macro_rules! Depcrate_errorUtf8Error {
() => {
// Module: crate::error
// Provides: {"Utf8Error"}
// Dependencies: {}
# [doc = " A UTF-8 validation error."] # [doc = ""] # [doc = " This occurs when attempting to convert a `ByteRecord` into a"] # [doc = " `StringRecord`."] # [doc = ""] # [doc = " The error includes the index of the field that failed validation, and the"] # [doc = " last byte at which valid UTF-8 was verified."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Utf8Error { # [doc = " The field index of a byte record in which UTF-8 validation failed."] field : usize , # [doc = " The index into the given field up to which valid UTF-8 was verified."] valid_up_to : usize , }
};
}
