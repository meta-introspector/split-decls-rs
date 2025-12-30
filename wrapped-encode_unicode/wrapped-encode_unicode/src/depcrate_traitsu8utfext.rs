// Generated macro for U8UtfExt (trait)
macro_rules! Depcrate_traitsU8UtfExt {
() => {
// Module: crate::traits
// Provides: {"U8UtfExt"}
// Dependencies: {}
# [doc = " Methods for working with `u8`s as UTF-8 bytes."] pub trait U8UtfExt { # [doc = " How many more bytes will you need to complete this codepoint?"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error is returned if the byte is not a valid start of an UTF-8"] # [doc = " codepoint:"] # [doc = ""] # [doc = " * `128..192`: [`UnexpectedContinuationByte`](error/enum.Utf8ErrorKind.html#variant.UnexpectedContinuationByte)"] # [doc = " * `245..`, `192` and `193`: [`NonUtf8Byte`](error/enum.Utf8ErrorKind.html#variant.NonUtf8Byte)  "] fn extra_utf8_bytes (self) -> Result < usize , Utf8Error > ; # [doc = " How many more bytes will you need to complete this codepoint?"] # [doc = ""] # [doc = " This function assumes that the byte is a valid UTF-8 start, and might"] # [doc = " return any value otherwise. (but the function is safe to call with any"] # [doc = " value and will return a consistent result)."] fn extra_utf8_bytes_unchecked (self) -> usize ; }
};
}
