// Generated macro for Error (enum)
macro_rules! Depcrate_base64Error {
() => {
// Module: crate::base64
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) enum Error { # [doc = " Given character is not valid in base64 alphabet."] InvalidCharacter (u8) , # [doc = " A padding character (`=`) appeared outside the final"] # [doc = " block of 4 characters."] PrematurePadding , # [doc = " The padding characters at the end of the input were invalid."] InvalidTrailingPadding , # [doc = " Not enough space in output buffer."] # [doc = ""] # [doc = " Use `decoded_length` to get an upper bound."] InsufficientOutputSpace , }
};
}
